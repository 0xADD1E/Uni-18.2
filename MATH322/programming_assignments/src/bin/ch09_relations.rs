#[macro_use]
extern crate log;
extern crate env_logger;
extern crate failure;
extern crate programming_assignments;
extern crate regex;

fn main(){
    run_main().unwrap();
}
fn run_main() -> Result<(), failure::Error> {
    env_logger::init();

    let file = programming_assignments::get_argv1()?;
    let file = std::path::Path::new(&file);
    let mut file = std::fs::File::open(file)?;
    let mut string = String::new();

    use std::io::Read;
    info!("Reading file {:?}", file);
    file.read_to_string(&mut string)?;
    debug!("Got string {:?}", string);

    let re = regex::Regex::new(r"\{((0|1)(, ?)?)+\}")?;
    let mat = re
        .captures_iter(&string)
        .filter_map(|cap| {
            trace!("Working on parsing regex match: {:?}", cap);
            cap.get(0)
        })
        .map(|rgx_mat| String::from(rgx_mat.as_str()))
        .map(|row| {
            row.chars()
                .filter(|c| c != &'{' && c != &'}' && c != &' ')
                .collect::<String>()
                .split(',')
                .map(|c| {
                    trace!("Parsing individual character (should be 0 or 1): '{}'", c);
                    match c {
                        "0" => false,
                        "1" => true,
                        _ => false,
                    }
                })
                .collect::<Vec<bool>>()
        })
        .collect::<Vec<Vec<bool>>>();
    debug!("Parsed file as {:?}", mat);

    let mat = compute_reflexive_closure(&mat);
    let mat = compute_symmetric_closure(&mat);
    let mat = compute_transitive_closure(&mat);
    let mat = mat
        .into_iter()
        .map(|sub| {
            sub.into_iter()
                .map(|entry| match entry {
                    true => '1',
                    false => '0',
                })
                .collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>();
    let format_str = format!("{:?}", mat)
        .replace("[", "{")
        .replace("]", "}")
        .replace("'", "");
    println!("{}", format_str);
    Ok(())
}
fn compute_reflexive_closure(mat: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    trace!("Starting reflexive closure computation");
    let mut new_mat = mat.clone(); //We could do this cleanly, but *meh*
    for i in 0..new_mat.len() {
        trace!("Matrix at {},{} was {}, now is true", i, i, new_mat[i][i]);
        new_mat[i][i] |= true;
    }
    new_mat
}
fn compute_symmetric_closure(mat: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let transpose_mat = transpose(&mat);
    let mut new_mat = mat.clone();
    for i in 0..new_mat.len() {
        for j in 0..new_mat.len() {
            trace!(
                "Matrix at {},{} was {}, now is {}",
                i,
                j,
                new_mat[i][j],
                new_mat[i][j] | transpose_mat[i][j]
            );
            new_mat[i][j] |= transpose_mat[i][j];
        }
    }
    new_mat
}
fn transpose(mat: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut new_mat = mat.clone();
    for i in 0..new_mat.len() {
        for j in 0..new_mat.len() {
            new_mat[i][j] = mat[j][i];
        }
    }
    new_mat
}
fn compute_transitive_closure(mat: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut new_mat = mat.clone();
    for i in 0..new_mat.len() {
        for j in 0..new_mat.len() {
            for k in 0..new_mat.len() {
                trace!("Working on i,k,j = {},{},{}", i, k, j);
                trace!("i,j is {}", new_mat[i][j]);
                trace!("i,k is {}", new_mat[i][k]);
                trace!("k,j is {}", new_mat[k][j]);
                new_mat[i][j] |= new_mat[i][k] && new_mat[k][j];
                debug!("New mat[i][j] is {}", new_mat[i][j]);
            }
        }
    }
    new_mat
}
