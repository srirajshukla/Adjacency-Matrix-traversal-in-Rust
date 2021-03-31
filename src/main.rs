use std::{collections::VecDeque, usize};

fn visit_cities_with_bfs(cities: &Vec<Vec<i32>>, start_city : usize, n : usize){
    let mut visited = vec![false;n];
    let all_visited = visited.iter().fold(true, |res, &t| res && t);

    let mut cities_to_visit : VecDeque<usize>= VecDeque::new();
    cities_to_visit.push_back(start_city);
    visited[start_city] = true;

    while !cities_to_visit.is_empty() && !all_visited{
        let current_city  = cities_to_visit.front().unwrap().clone() as usize;
        cities_to_visit.pop_front();

        println!("Currently visiting: {}", &current_city);

        for i in 0..n{
            if !visited[i] && cities[current_city][i]==1 {
                cities_to_visit.push_back(i);
                visited[i] = true;
            }
        }
    }
}


fn dfs(cities: &Vec<Vec<i32>>, visited : &mut Vec<bool>, i : usize, n:usize){
    visited[i] = true;
    println!("Currently visiting: {}", i);

    for j in 0..n{
        if !visited[j] && cities[i][j]==1 {
            dfs(cities, visited, j, n);
        }
    }
}

fn check_connected_dfs(cities: Vec<Vec<i32>>, start_city : usize, n : usize) -> bool{
    let mut visited = vec![false; n];
    dfs(&cities, &mut visited, start_city, n);

    for visit in visited{
        if visit==false{
            return false;
        }
    }
    return true;
}

fn insert_path(cities:&mut Vec<Vec<i32>>, start_city : usize, end_city : usize) {
    cities[start_city][end_city] = 1;
    cities[end_city][start_city] = 1;
}

fn print_adj_mat(cities: &Vec<Vec<i32>>){
    for x in cities{
        for y in x{
            print!("{} ", y);
        }
        println!();
    }
}

fn main() {
    let n = 6;
    let city:Vec<i32> = vec![0; n];
    let mut cities = vec![city; n];


    insert_path(&mut cities, 1, 2);
    insert_path(&mut cities, 1, 3);
    insert_path(&mut cities, 1, 4);
    insert_path(&mut cities, 3, 2);
    insert_path(&mut cities, 5, 2);

    print_adj_mat(&cities);

    visit_cities_with_bfs(&cities, 1, n);

    println!("\nDFS:\n");

    let connected = check_connected_dfs(cities, 1, n);
    print!("The given graph is {} connected",connected);
    
}
