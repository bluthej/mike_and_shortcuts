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

    walk(0, 0, &shortcuts, &mut energies);

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
