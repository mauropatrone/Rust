use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut note_map: HashMap<&str, u32> = HashMap::new();
    for s in note {
        note_map.entry(*s)
           .and_modify(|e| { *e += 1 })
           .or_insert(1);
    }
    for s in magazine {
        note_map.entry(*s)
           .and_modify(|e| { *e = e.saturating_sub(1); });
    }
    for val in note_map.values() {
        if val != &0u32 {
            return false
        }
    }
    true
}

#[test]
fn test_false() {
    let magazine = "two times three is not four".split_whitespace().collect::<Vec<&str>>();
    let note = "two times two is four".split_whitespace().collect::<Vec<&str>>();
    assert_eq!(can_construct_note(&magazine, &note), false);
}

#[test]
fn test_true() {
    let magazine = "Astronomer Amy Mainzer spent hours chatting with Leonardo DiCaprio for Netflix's 'Don't Look Up'".split_whitespace().collect::<Vec<&str>>();
    let note = "Amy Mainzer chatting with Leonardo DiCaprio".split_whitespace().collect::<Vec<&str>>();
    assert_eq!(can_construct_note(&magazine, &note), true);
}

#[test]
fn test_2() {
    //let magazine = "two times three is not four".split_whitespace().collect::<Vec<&str>>();
    let note = "two times two is four".split_whitespace().collect::<Vec<&str>>();
    let mut note_map: HashMap<&str, u32> = HashMap::new();
    for s in &note {
        note_map.entry(*s)
           .and_modify(|e| { *e += 1 })
           .or_insert(1);
    }
    assert_eq!(note_map.get(note[0]), Some(&2u32));
    assert_eq!(note_map.get(note[1]), Some(&1u32));
}

#[test]
fn test_3() {
    let magazine = "two two two times three is not four".split_whitespace().collect::<Vec<&str>>();
    let note = "two times two is four".split_whitespace().collect::<Vec<&str>>();
    let mut note_map: HashMap<&str, u32> = HashMap::new();
    for s in &note {
        note_map.entry(*s)
           .and_modify(|e| { *e += 1 })
           .or_insert(1);
    }
    for s in &magazine {
        note_map.entry(*s)
           .and_modify(|e| { *e = e.saturating_sub(1); });
    }
    assert_eq!(note_map.get(note[0]), Some(&0u32));
    assert_eq!(note_map.get(note[1]), Some(&0u32));
}

fn main() {
    let note = "two times two is four".split_whitespace().collect::<Vec<&str>>();
    println!("{}",note[1]);
}
