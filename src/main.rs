use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (n, shortcuts) = read_from_stdin()?;
    let n = n.trim().parse::<usize>()?;
    let shortcuts: Result<Vec<usize>, _> = shortcuts
        .trim()
        .split(" ")
        .map(|a| a.parse::<usize>())
        .collect();
    let shortcuts = shortcuts?;

    let min_shortcuts = shortcuts.iter().min().unwrap().clone();

    let mut energies: Vec<usize> = (0..n).collect();

    let mut i = 0;
    let mut e = 1;
    while i < n {
        if shortcuts[i] > i + 1 {
            i = shortcuts[i] - 1;
            energies[i] = e;
            e += 1;
        } else {
            i += 1;
        }
    }
    dbg!(&energies);

    //for i in 1..=n {
    //if i <= min_shortcuts {
    //let e = (i - 1).min(1 + min_shortcuts - i);
    //energies[i - 1] = e;
    //continue;
    //}
    //}
    //dbg!(energies);
    Ok(())
}

fn read_from_stdin() -> io::Result<(String, String)> {
    let mut n = String::new();
    io::stdin().read_line(&mut n)?;

    let mut shortcuts = String::new();
    io::stdin().read_line(&mut shortcuts)?;

    Ok((n, shortcuts))
}
