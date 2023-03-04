use clap::{arg, Args};

use crate::Error;

fn c_rand_u8() -> u8 {
    unsafe { rand() }
}

extern "C" {
    fn rand() -> u8;
}

/// Famous Russian gun game that blows brains
#[derive(Args)]
pub struct Options {
    /// Barrel size
    #[arg(short, long, value_name = "NUM", default_value_t = 6)]
    barrel: u8,
    /// Bullet count
    #[arg(short = 'l', long, value_name = "NUM", default_value_t = 1)]
    bullets: u8,
    /// Clicks number
    #[arg(short, long, value_name = "NUM", default_value_t = 6)]
    clicks: u8,
}

#[derive(Debug)]
struct Gun {
    barrel: Vec<bool>,
    hammer_pos: u8,
}

impl Gun {
    // Create new Gun instance
    pub fn new(barrel_size: u8, mut bullets: u8) -> Self {
        match barrel_size {
            0 => panic!("0 Barrel size!"),
            _ => println!("Barrel with size {} opened", &barrel_size),
        }
        let mut barrel = vec![false; barrel_size as usize];
        let mut hammer_pos: u8 = 0;
        match bullets {
            0 => panic!("0 Bullets!"),
            _ => {
                if bullets > barrel_size {
                    panic!("Cannot fit {} bullets inside barrel", &bullets);
                } else {
                    println!("Barrel loading with {} bullets", &bullets);
                    while bullets > 0 {
                        hammer_pos = c_rand_u8() % barrel_size;
                        if !barrel[hammer_pos as usize] {
                            barrel[hammer_pos as usize] = true;
                            bullets -= 1;
                        }
                    }
                }
            }
        }
        Self { barrel, hammer_pos }
    }

    // Trigger gun an fire round if loaded in barrel
    pub fn trigger(&mut self) {
        self.hammer_pos += 1;
        let real_pos = self.hammer_pos as usize % self.barrel.len();
        if self.barrel[real_pos] {
            self.barrel[real_pos] = false;
            println!("BAAANNG!!");
        } else {
            println!("*Click*, {real_pos} empty");
        }
    }
}

/// # Errors
///
/// Will return `Err` string is empty
pub fn main(opts: &Options) -> Result<(), Error> {
    let mut gun = Gun::new(opts.barrel, opts.bullets);

    for _ in 0..opts.clicks {
        gun.trigger();
    }

    Ok(())
}
