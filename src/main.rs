fn main() {
    let mut player_hp: i32 = 100;
    let total_floors = 5;

    // Tracks how the run ended so we can print the right final message later.
    let mut outcome = "escaped"; // will become "died" or "fled" if either happens

    // --- FOR LOOP: cares about "which floor am I on?" ---
    // It's labeled 'floors so that a flee action deep inside a fight can jump
    // all the way out of the tower in one shot, not just out of the fight.
    'floors: for floor in 1..=total_floors {
        println!("\n--- Floor {} ---", floor);

        // Enemies get a bit tougher the higher up you go.
        let mut enemy_hp: i32 = 20 + floor * 10;
        println!("An enemy blocks your path! Enemy HP: {}", enemy_hp);

        let mut turn_count = 0;

        // --- WHILE LOOP: cares about "is this fight still going?" ---
        // Keeps running as long as both the enemy and the player are alive.
        while enemy_hp > 0 && player_hp > 0 {
            turn_count += 1;

            // --- LOOP: cares about "what happens this one turn?" ---
            // Nested inside the while loop. Every pass through here is a single turn.
            loop {
                // Simple deterministic "miss": every 4th turn the attack fails.
                // continue sends us back to the top of this same turn loop,
                // so the turn is redone instead of ending.
                if turn_count % 4 == 0 {
                    println!("Turn {}: You swing and miss!", turn_count);
                    turn_count += 1;
                    continue;
                }

                // If HP is critically low, flee the entire tower instead of fighting on.
                // break 'floors jumps straight out of the outer for loop,
                // skipping the rest of this fight and all remaining floors.
                if player_hp <= 15 {
                    println!("Turn {}: HP critical! You panic and flee the tower!", turn_count);
                    outcome = "fled";
                    break 'floors;
                }

                // Normal action: attack the enemy.
                let damage_dealt = 12;
                enemy_hp -= damage_dealt;
                println!(
                    "Turn {}: You attack for {} damage. Enemy HP: {}",
                    turn_count,
                    damage_dealt,
                    enemy_hp.max(0)
                );

                // If that attack finished the enemy off, don't let it hit back.
                if enemy_hp <= 0 {
                    break; // ends the turn loop, control goes back to the while loop
                }

                // Enemy strikes back.
                let damage_taken = 8;
                player_hp -= damage_taken;
                println!(
                    "The enemy hits back for {} damage. Your HP: {}",
                    damage_taken,
                    player_hp.max(0)
                );

                // End the turn normally. Control returns to the while loop,
                // which re-checks whether the fight should keep going.
                break;
            }
        }

        // After the fight loop ends (enemy dead, player dead, or already fled),
        // figure out what happened and whether to continue to the next floor.
        if player_hp <= 0 {
            println!("You died on floor {}.", floor);
            outcome = "died";
            break 'floors;
        } else if enemy_hp <= 0 {
            println!("Floor {} cleared! Moving up...", floor);
            // for loop naturally continues to the next floor here
        }
    }

    // --- FINAL MESSAGE ---
    println!("\n--- Game Over ---");
    if outcome == "died" {
        println!("You perished inside the Obsidian Tower.");
    } else if outcome == "fled" {
        println!("You fled the tower with your life, but barely.");
    } else {
        println!("You escaped the Obsidian Tower victorious!");
    }
}
