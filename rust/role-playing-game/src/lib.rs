pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        if self.health > 0 {
            None
        } else {
            Some(Player {
                health: 100,
                mana: {
                    if self.level > 9 {
                        Some(100)
                    } else {
                        None
                    }
                },
                level: self.level,
            })
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match self.mana {
            None => {
                self.health = {
                    if mana_cost > self.health {
                        0
                    } else {
                        self.health - mana_cost
                    }
                };
                0
            }
            Some(current_mana) => {
                if current_mana < mana_cost {
                    0
                } else {
                    self.mana = Some(current_mana - mana_cost);
                    mana_cost * 2
                }
            }
        }
    }
}
