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
    Placed {
        x: Size,
        y: Size
    },
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

impl Game {
    pub fn new(
        width: Size, 
        height: Size
    ) -> Self {
        Game {
            width: width,
            height: height,
            pawns: vec![],
            pawns_ownerships: HashMap::new()
        }
    }

    pub fn create_pawns_from<'a, I>(&mut self, tuples: I) -> Result<(), RulesError>
    where 
        I: Iterator<Item=&'a(Id, PawnState)>
    {
        for tuple in tuples {
            match self.create_pawn_for_player(tuple.0, tuple.1) {
                Err(err) => return Err(err),
                _ => ()
            };
        }
        Ok(())
    }

    pub fn create_pawn_for_player(&mut self, player_id: Id, state: PawnState) -> Result<(), RulesError> {
        match self.create_pawn(state) {
            Ok(pawn) => Ok(self.give_pawn(player_id, pawn)),
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
            (PawnState::Unplaced, PawnState::Placed{x, y}) => {
                if self.is_position_free(x, y) {
                    pawn.state = state;
                    Ok(())
                } else {
                    Err(RulesError::PositionTaken)
                }
            },
            _ => Err(RulesError::IllegalStateTransition)
        }
    }

    pub fn placed_pawns(&self) -> impl Iterator<Item = &Pawn> {
        self.pawns.iter()
            .filter(
                |&pawn| matches!(pawn.state, PawnState::Placed{x:_, y:_})
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

    fn is_position_free(&self, x: Size, y: Size) -> bool {
        let mut found = false;
        for p in self.pawns.iter() {
            found = match p.state {
                PawnState::Placed{x: xb, y: yb} => xb == x && yb == y,
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
        Pawn { id: id, state: state }
    }
}