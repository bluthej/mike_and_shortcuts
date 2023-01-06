use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    io::stdin().read_line(&mut input)?;
    let shortcuts = input.lines().skip(1).next().unwrap();
    let shortcuts = shortcuts
        .trim()
        .split(" ")
        .map(|a| a.parse::<usize>())
        .collect::<Result<Vec<usize>, _>>()?;
    let shortcuts: Vec<usize> = shortcuts.iter().map(|i| i - 1).collect();
    let n = shortcuts.len();

    let mut energies: Vec<Option<usize>> = vec![None; n];

    //walk(0, 0, &shortcuts, &mut energies);
    walk_iter(&shortcuts, &mut energies);

    println!(
        "{}",
        energies
            .into_iter()
            .filter_map(|e| e.and_then(|v| Some(v.to_string())))
            .collect::<Vec<String>>()
            .join(" ")
    );

    Ok(())
}

fn walk_iter(shortcuts: &[usize], energies: &mut [Option<usize>]) {
    energies[0] = Some(0);
    let n = energies.len();

    let mut stack: Vec<(usize, usize)> = vec![(0, 0)];
    while !stack.is_empty() {
        let (current, current_energy) = stack.pop().unwrap();
        let next_energy = current_energy + 1;

        let next = current.saturating_sub(1);
        if next > 1 && (energies[next].is_none() || energies[next] > Some(next_energy)) {
            energies[next] = Some(next_energy);
            stack.push((next, next_energy));
        }

        let next = current + 1;
        if next < n && (energies[next].is_none() || energies[next] > Some(next_energy)) {
            energies[next] = Some(next_energy);
            stack.push((next, next_energy));
        }

        let next = shortcuts[current];
        if energies[next].is_none() || energies[next] > Some(next_energy) {
            energies[next] = Some(next_energy);
            stack.push((next, next_energy));
        }
    }
}

#[allow(dead_code)]
fn walk(current: usize, energy: usize, shortcuts: &[usize], energies: &mut [Option<usize>]) {
    energies[current] = Some(energy);

    let next_energy = energy + 1;

    let next = shortcuts[current];
    if energies[next].is_none() || energies[next] > Some(next_energy) {
        walk(next, next_energy, shortcuts, energies);
    }

    let next = current + 1;
    let n = energies.len();
    if next < n && (energies[next].is_none() || energies[next] > Some(next_energy)) {
        walk(next, next_energy, shortcuts, energies);
    }

    let next = current.saturating_sub(1);
    if next > 1 && (energies[next].is_none() || energies[next] > Some(next_energy)) {
        walk(next, next_energy, shortcuts, energies);
    }
}
