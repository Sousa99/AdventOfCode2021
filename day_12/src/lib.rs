use std::collections::HashMap;

// ================================================== STRUCTS ==================================================

type CaveID = String;

fn get_start_id() -> CaveID { String::from("start") }
fn get_end_id() -> CaveID { String::from("end") }

#[derive(PartialEq)]
enum CaveType { Start, Small, Big, End }

struct Cave {
    id:         CaveID,
    cave_type:  CaveType,
}

pub struct Map {
    caves:      HashMap<CaveID, Cave>,
    cave_links: HashMap<CaveID, Vec<CaveID>>,
}

type Path = Vec<CaveID>;

// ================================================== AUX FUNCTIONS ==================================================

fn count_element<T: PartialEq>(vector: &Vec<T>, element: &T) -> u64 {

    let mut count : u64 = 0;
    for elem in vector {
        if elem == element {
            count = count + 1;
        }
    }

    return count;
}

// ================================================== IMPLEMENTATIONS ==================================================

impl Cave {

    fn new(id: CaveID) -> Cave {

        let cave_type : CaveType;
        if id == get_start_id() { cave_type = CaveType::Start }
        else if id == get_end_id() { cave_type = CaveType::End }
        else if id == id.to_uppercase() { cave_type = CaveType::Big }
        else { cave_type = CaveType::Small }

        Cave {
            id: id,
            cave_type: cave_type
        }
    }
}

impl Map {

    pub fn new(links: Vec<(CaveID, CaveID)>) -> Map {

        let mut caves : HashMap<CaveID, Cave> = HashMap::new();
        let mut cave_links : HashMap<CaveID, Vec<CaveID>> = HashMap::new();

        for (link_from, link_to) in links.into_iter() {

            if !caves.contains_key(&link_from) { caves.insert(link_from.clone(), Cave::new(link_from.clone())); }
            if !caves.contains_key(&link_to) { caves.insert(link_to.clone(), Cave::new(link_to.clone())); }

            if cave_links.contains_key(&link_from) {

                let known_links = cave_links.get_mut(&link_from).unwrap();
                known_links.push(link_to.clone());

            } else { cave_links.insert(link_from.clone(), vec!(link_to.clone())); }

            if cave_links.contains_key(&link_to) {

                let known_links = cave_links.get_mut(&link_to).unwrap();
                known_links.push(link_from.clone());

            } else { cave_links.insert(link_to.clone(), vec!(link_from.clone())); }
        }

        Map {
            caves: caves,
            cave_links: cave_links,
        }
    }

    fn compute_max_number_of_small_revisited(&self, path: &Path) -> Option<(CaveID, u64)> {

        let mut counts : HashMap<CaveID, u64> = HashMap::new();
        for cave_id in path {

            if counts.contains_key(cave_id) {
                let current_value = counts.get_mut(cave_id).unwrap();
                *current_value = *current_value + 1u64;
            } else { counts.insert(cave_id.clone(), 1); }
        }

        let max_repetitions_option = counts.into_iter()
            .filter(|(cave_id, _)| self.caves.get(cave_id).unwrap().cave_type == CaveType::Small)
            .max_by_key(|(_, count)| *count);

        return max_repetitions_option;
    }

    pub fn compute_paths(&self, allow_one_rep: bool) -> Vec<Path> {

        let mut valid_paths : Vec<Path> = Vec::new();
        let mut current_paths : Vec<Path> = vec!(vec!(get_start_id()));
        while current_paths.len() != 0 {

            // println!();
            let current_path = current_paths.pop().unwrap();
            // println!("Current Path: {:?}", current_path);
            let last_cave = current_path.last().unwrap();

            for possible_cave_id in self.cave_links.get(last_cave).unwrap() {

                // println!("\tAdding: {}", possible_cave_id);
                let possible_cave = self.caves.get(possible_cave_id).unwrap();

                // Exceptions
                let max_repetition_option = self.compute_max_number_of_small_revisited(&current_path);
                if !allow_one_rep && possible_cave.cave_type == CaveType::Small && current_path.contains(possible_cave_id) { continue; }
                if allow_one_rep && possible_cave.cave_type == CaveType::Small && current_path.contains(possible_cave_id) &&
                    (count_element(&current_path, possible_cave_id) >= 2 || ( max_repetition_option.is_some() && max_repetition_option.unwrap().1 >= 2))
                        { continue; }
                if possible_cave.cave_type == CaveType::Start { continue; }

                // println!("Added somewhere!");
                // Add path
                let mut new_path = current_path.clone();
                new_path.push(possible_cave_id.clone());

                if possible_cave.cave_type == CaveType::End { valid_paths.push(new_path); }
                else { current_paths.push(new_path); }
            }
        }


        return valid_paths;
    }
}