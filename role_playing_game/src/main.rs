fn main() {
    let dead_player = Player { health: 0, mana: None, level: 2 };
    let res1 = dead_player.revive();
    println!("res1: {:?}", res1);

    let dead_player = Player { health: 0, mana: None, level: 10 };
    let res2 = dead_player.revive();
    println!("res2: {:?}", res2);

    let alive_player = Player { health: 1, mana: Some(15), level: 11 };
    let res3 = alive_player.revive();
    println!("res3: {:?}", res3);
    
    let mut not_a_wizard_yet = Player { health: 79, mana: None, level: 9 };
    assert_eq!(not_a_wizard_yet.cast_spell(5), 0);
    assert_eq!(not_a_wizard_yet.health, 74);
    
    let mut not_a_wizard_yet = Player { health: 79, mana: Some(30), level: 9 };
    assert_eq!(not_a_wizard_yet.cast_spell(5), 25);
    assert_eq!(not_a_wizard_yet.mana, Some(25));
}

#[derive(Debug)]
pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        match self.health {
            0 => Some(Player{
                health: 100,
                mana: if self.level >= 10 { Some(100) } else { None }, 
                ..*self }),
            _ => None
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match self.mana {
            Some(cost) => if mana_cost > cost { 0 } else {
                self.mana = Some(cost - mana_cost);
                2 * mana_cost 
            },
            None => if self.health < mana_cost {
                self.health = 0;
                return 0;
            } else {
                self.health -= mana_cost;
                return 0;
            },
        }
    }
}

