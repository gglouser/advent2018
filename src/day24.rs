use regex::{Match, Regex};
use std::collections::HashMap;

#[derive(Clone, PartialEq, Debug)]
struct Group {
    units: u32,
    hit_points: u32,
    attack_dmg: u32,
    attack_type: String,
    initiative: u32,
    weak: Vec<String>,
    immune: Vec<String>,
}

impl Group {
    fn effective_power(&self) -> u32 {
        self.units * self.attack_dmg
    }

    // Calculate effective damage this group would do to defender.
    fn effective_damage(&self, defender: &Group) -> u32 {
        if defender.immune.contains(&self.attack_type) {
            0
        } else if defender.weak.contains(&self.attack_type) {
            self.effective_power() * 2
        } else {
            self.effective_power()
        }
    }

    // Resolve attack on this group, returning number of units killed.
    fn resolve_attack(&mut self, damage: u32) -> u32 {
        let kills = damage / self.hit_points;
        let kills = kills.min(self.units);
        self.units -= kills;
        kills
    }
}

fn parse_input(s: &str) -> Vec<Vec<Group>> {
    let re_group = Regex::new(r"(\d+) units each with (\d+) hit points (?:\((.*)\) )?with an attack that does (\d+) (\w+) damage at initiative (\d+)").unwrap();
    let mut lines = s.lines();
    let mut armies = Vec::new();
    while let Some(_line) = lines.next() {
        let mut groups = Vec::new();
        while let Some(line) = lines.next() {
            if line == "" { break; }
            if let Some(c) = re_group.captures(line) {
                let units = c[1].parse().unwrap();
                let hit_points = c[2].parse().unwrap();
                let (weak, immune) = parse_modifiers(c.get(3));
                let attack_dmg = c[4].parse().unwrap();
                let attack_type = String::from(&c[5]);
                let initiative = c[6].parse().unwrap();
                groups.push(Group { units, hit_points, attack_dmg, attack_type, initiative, weak, immune });
            } else {
                println!("invalid group description: {}", line);
            }
        }
        armies.push(groups);
    }
    armies
}

fn parse_modifiers(mods: Option<Match>) -> (Vec<String>, Vec<String>) {
    let mut weak = vec![];
    let mut immune = vec![];
    if let Some(m) = mods {
        let parts = m.as_str().split("; ");
        for part in parts {
            let subp: Vec<&str> = part.split(" to ").collect();
            let modifier = subp[0];
            let dmg_types = subp[1].split(", ").map(String::from);
            if modifier == "weak" {
                weak.extend(dmg_types);
            } else if modifier == "immune" {
                immune.extend(dmg_types);
            } else {
                println!("invalid damage modifier: {}", modifier);
            }
        }
    }
    (weak, immune)
}

fn choose_targets(attackers: &[Group], defenders: &[Group]) -> HashMap<usize, usize> {
    // Determine target selection order
    let mut select_order: Vec<(usize, &Group)> = attackers.iter().enumerate()
        .filter(|&(_, a)| a.units > 0)
        .collect();
    select_order.sort_unstable_by_key(|&(_, a)| (a.effective_power(), a.initiative));
    select_order.reverse();

    // Select targets
    let mut targeted = vec![false; defenders.len()];
    select_order.into_iter().filter_map(|(i, attacker)|
        defenders.iter().enumerate()
            .filter(|&(j, def)| !targeted[j] && def.units > 0 && attacker.effective_damage(def) > 0)
            .max_by_key(|&(_, def)| (attacker.effective_damage(def), def.effective_power(), def.initiative))
            .map(|(j, _)| {
                targeted[j] = true;
                (i, j)
            })
    ).collect()
}

fn battle_round(armies: &mut Vec<Vec<Group>>) -> u32 {
    let targets = vec![
        choose_targets(&armies[0], &armies[1]),
        choose_targets(&armies[1], &armies[0]),
    ];

    let mut attack_order: Vec<(usize,usize)> = armies.iter().enumerate()
        .flat_map(|(a, army)| (0..army.len()).map(move |g| (a, g)))
        .collect();
    attack_order.sort_unstable_by_key(|&(a, g)| armies[a][g].initiative);
    attack_order.reverse();

    let mut total_kills = 0;
    for (a, g) in attack_order {
        if armies[a][g].units == 0 { continue; }
        if let Some(&d) = targets[a].get(&g) {
            let def_army = 1 - a;
            let damage = armies[a][g].effective_damage(&armies[def_army][d]);
            let kills = armies[def_army][d].resolve_attack(damage);
            total_kills += kills;

            // println!("{} group {} attacks defending group {}, killing {} units",
            //     if a == 1 { "Infection" } else { "Immune System" },
            //     g, d, kills);
        }
    }
    total_kills
}

fn simulate_battle(mut armies: Vec<Vec<Group>>) -> (u32, u32) {
    let mut round = 0;
    loop {
        // println!();
        // for a in 0..2 {
        //     if a == 0 { println!("Immune System:"); } else { println!("Infection:"); }
        //     for (i, g) in armies[a].iter().enumerate() {
        //         if g.units > 0 {
        //             println!("Group {} contains {} units", i, g.units);
        //         }
        //     }
        // }
        // println!();

        let kills = battle_round(&mut armies);
        let imm_alive = armies[0].iter().map(|g| g.units).sum();
        let inf_alive = armies[1].iter().map(|g| g.units).sum();
        if imm_alive == 0 || inf_alive == 0 {
            return (imm_alive, inf_alive);
        }
        if kills == 0 {
            println!("draw at round {} :: {} - {}", round, imm_alive, inf_alive);
            return (0,0);
        }
        round += 1;
    }
}

fn give_boost(army: &[Group], boost: u32) -> Vec<Group> {
    let mut new_army = army.to_vec();
    for g in new_army.iter_mut() {
        g.attack_dmg += boost;
    }
    new_army
}

fn solve(input: &str) -> (u32, u32) {
    let armies = parse_input(input);

    let (_, alive) = simulate_battle(armies.clone());

    let imm_win = (1..).map(|b| {
        println!("testing boost {} ...", b);
        let army_imm_boosted = give_boost(&armies[0], b);
        let (imm_alive, _) = simulate_battle(vec![army_imm_boosted, armies[1].clone()]);
        if imm_alive > 0 { println!("immune system won with {} units with boost {}", imm_alive, b); }
        imm_alive
    }).find(|&a| a > 0).unwrap();

    (alive, imm_win)
}

pub fn run(input: &str) {
    let (part1, part2) = solve(input);
    println!("the solution to part 1 is {}", part1);
    println!("the solution to part 2 is {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE : &'static str = "\
Immune System:
17 units each with 5390 hit points (weak to radiation, bludgeoning) with \
 an attack that does 4507 fire damage at initiative 2
989 units each with 1274 hit points (immune to fire; weak to bludgeoning, \
 slashing) with an attack that does 25 slashing damage at initiative 3

Infection:
801 units each with 4706 hit points (weak to radiation) with an attack \
 that does 116 bludgeoning damage at initiative 1
4485 units each with 2961 hit points (immune to radiation; weak to fire, \
 cold) with an attack that does 12 slashing damage at initiative 4
";

    #[test]
    fn parsing() {
        assert_eq!(parse_input(EXAMPLE), vec![
            vec![
                Group {
                    units: 17,
                    hit_points: 5390,
                    attack_dmg: 4507,
                    attack_type: String::from("fire"),
                    initiative: 2,
                    weak: vec![String::from("radiation"), String::from("bludgeoning")],
                    immune: vec![],
                },
                Group {
                    units: 989,
                    hit_points: 1274,
                    attack_dmg: 25,
                    attack_type: String::from("slashing"),
                    initiative: 3,
                    weak: vec![String::from("bludgeoning"), String::from("slashing")],
                    immune: vec![String::from("fire")],
                },
            ],
            vec![
                Group {
                    units: 801,
                    hit_points: 4706,
                    attack_dmg: 116,
                    attack_type: String::from("bludgeoning"),
                    initiative: 1,
                    weak: vec![String::from("radiation")],
                    immune: vec![],
                },
                Group {
                    units: 4485,
                    hit_points: 2961,
                    attack_dmg: 12,
                    attack_type: String::from("slashing"),
                    initiative: 4,
                    weak: vec![String::from("fire"), String::from("cold")],
                    immune: vec![String::from("radiation")],
                },
            ]
        ]);
    }

    #[test]
    fn example() {
        let (part1, part2) = solve(EXAMPLE);
        assert_eq!(5216, part1);
        assert_eq!(51, part2);
    }

    #[cfg(feature="test_real_input")]
    #[test]
    fn real_input() {
        let input = include_str!("../inputs/day24.txt");
        let x = solve(&input);
        assert_eq!(include_str!("../outputs/day24.txt"),
                   format!("{:?}", x));
    }
}
