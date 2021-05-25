use std::convert::TryFrom;
use std::collections::HashMap;

pub type Pawns = Vec<Pawn>;
pub type Size = u8;
pub type Id = u8;

pub struct Game {
    pub height: Size,
    pub width: Size,
    pub pawns: Pawns,
    pub pawns_ownerships: HashMap<Id, Id>
}

#[derive(Copy, Clone, Debug)]
pub enum PawnState {
    Placed(Position),
    Unplaced
}

#[derive(Copy, Clone)]
pub struct Pawn {
    pub id: Id,
    pub state: PawnState
}

#[derive(Debug)]
pub enum RulesError {
    PositionTaken,
    IllegalStateTransition
}

#[derive(Copy, Clone, Debug)]
pub struct Position {
    pub x: Size,
    pub y: Size
}

impl Game {
    pub fn new(
        width: Size, 
        height: Size
    ) -> Self {
        Game {
            width,
            height,
            pawns: vec![],
            pawns_ownerships: HashMap::new()
        }
    }

    pub fn create_pawns_from<'a, I>(&mut self, tuples: I) -> Result<(), RulesError>
    where 
        I: Iterator<Item=&'a(Id, PawnState)>
    {
        for tuple in tuples {
            if let Err(err) = self.create_pawn_for_player(tuple.0, tuple.1) {
                return Err(err);
            }
        }
        Ok(())
    }

    pub fn create_pawn_for_player(&mut self, player_id: Id, state: PawnState) -> Result<(), RulesError> {
        match self.create_pawn(state) {
            Ok(pawn) => {
                self.give_pawn(player_id, pawn);
                Ok(())
            },
            Err(err) => Err(err),
        }
    }

    fn create_pawn(&mut self, state: PawnState) -> Result<Pawn, RulesError> {
        let id = self.gen_pawn_id();

        let mut pawn = Pawn::new(
            id,
            PawnState::Unplaced
        );

        match self.set_state(&mut pawn, state) {
            Ok(()) => {
                self.pawns.push(pawn);
                Ok(pawn)
            },
            Err(err) => Err(err)
        }
    }

    fn set_state(&self, pawn: &mut Pawn, state: PawnState) -> Result<(), RulesError> {
        match (pawn.state, state) {
            (PawnState::Unplaced, PawnState::Unplaced) => Ok(()),
            (PawnState::Unplaced, PawnState::Placed(Position{ x, y })) => {
                if !self.is_position_ofb(x, y) {
                    let (x, y) = self.position_pacman(x, y);
                    pawn.state = PawnState::Placed(Position{ x, y });
                    return Ok(());
                }
                if !self.is_position_free(x, y) {
                    return Err(RulesError::PositionTaken);
                }
                pawn.state = state;
                Ok(())
            },
            _ => Err(RulesError::IllegalStateTransition)
        }
    }

    pub fn placed_pawns(&self) -> impl Iterator<Item = &Pawn> {
        self.pawns.iter()
            .filter(
                |&pawn| matches!(pawn.state, PawnState::Placed(_))
            )
    }

    pub fn unplaced_pawns(&self) -> impl Iterator<Item = &Pawn> {
        self.pawns.iter()
            .filter(
                |&pawn| matches!(pawn.state, PawnState::Unplaced)
            )
    }

    pub fn who_owns_pawn(&self, pawn_id: Id) -> Option<&Id> {
        self.pawns_ownerships.get(&pawn_id)
    }

    fn give_pawn(&mut self, player_id: Id, pawn: Pawn) {
        self.pawns_ownerships.insert(pawn.id, player_id);
    }

    fn is_position_ofb(&self, x: Size, y: Size) -> bool {
        x < self.width && y < self.height
    }

    fn position_pacman(&self, x: Size, y: Size) -> (Size, Size) {
        (x % self.width, y % self.height)
    }

    fn is_position_free(&self, x: Size, y: Size) -> bool {
        let mut found = false;
        for p in self.pawns.iter() {
            found = match p.state {
                PawnState::Placed(Position{x: xb, y: yb}) => xb == x && yb == y,
                _ => false
            };
            if found {
                break;
            }
        };
        !found
    }

    fn gen_pawn_id(&self) -> Id {
        Id::try_from(self.pawns.len()).unwrap()
    }
}

impl Pawn {
    fn new(id: Id, state: PawnState) -> Self {
        Pawn { id, state }
    }
}