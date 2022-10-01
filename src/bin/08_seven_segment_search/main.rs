use itertools::Itertools;
use std::collections::BTreeMap;

// One entry of puzzle input
#[derive(Debug)]
struct Entry {
    signal_patterns: Vec<String>,
    outputs: Vec<String>,
}


// Reads one entry from stdin
fn read_entry() -> Option<Entry> {
    let mut line = String::new();
    if std::io::stdin().read_line(&mut line).unwrap() == 0 {
        return None;
    }
    let mut entry = Entry {
        signal_patterns: Vec::new(),
        outputs: Vec::new(),
    };
    let mut dst = &mut entry.signal_patterns;
    for token in line.split_ascii_whitespace().into_iter() {
        if token == "|" {
            dst = &mut entry.outputs;
            continue;
        }
        dst.push(String::from(token));
    }
    Some(entry)
}

// Describes an arrangement of wires connected to the segment display.
#[derive(Debug)]
struct Arrangement {
    wire_targets: Vec<char>,
    segment_patterns: Vec<String>,
    normalized_id: u128
}

impl Arrangement {

    #[allow(non_upper_case_globals)]
    fn new(wire_targets: Vec<char>) -> Self {
        let mut arr = Arrangement {
            wire_targets: wire_targets,
            segment_patterns: Vec::new(),
            normalized_id: 0u128,
        };

        // Wire normally destined for segment a is at position 0
        // in the arrangement. Wire normally destined for segment b
        // is at position 1, etc.
        const a: usize = 0;
        const b: usize = 1;
        const c: usize = 2;
        const d: usize = 3;
        const e: usize = 4;
        const f: usize = 5;
        const g: usize = 6;

        // Push the segments that are lit for each numeral. E.g. segment_patterns[2]
        // will hold the segments that light up when the wires intend to signal
        // the numeral 2.
        arr.segment_patterns.push([ // 0
            arr.wire_targets[a],
            arr.wire_targets[b],
            arr.wire_targets[c],
            arr.wire_targets[e],
            arr.wire_targets[f],
            arr.wire_targets[g],
        ].iter().collect());

        arr.segment_patterns.push([ // 1
            arr.wire_targets[c],
            arr.wire_targets[f],
        ].iter().collect());

        arr.segment_patterns.push([ // 2
            arr.wire_targets[a],
            arr.wire_targets[c],
            arr.wire_targets[d],
            arr.wire_targets[e],
            arr.wire_targets[g],
        ].iter().collect());

        arr.segment_patterns.push([ // 3
            arr.wire_targets[a],
            arr.wire_targets[c],
            arr.wire_targets[d],
            arr.wire_targets[f],
            arr.wire_targets[g],
        ].iter().collect());
        
        arr.segment_patterns.push([ // 4
            arr.wire_targets[b],
            arr.wire_targets[c],
            arr.wire_targets[d],
            arr.wire_targets[f],
        ].iter().collect());

        arr.segment_patterns.push([ // 5
            arr.wire_targets[a],
            arr.wire_targets[b],
            arr.wire_targets[d],
            arr.wire_targets[f],
            arr.wire_targets[g],
        ].iter().collect());

        arr.segment_patterns.push([ // 6
            arr.wire_targets[a],
            arr.wire_targets[b],
            arr.wire_targets[d],
            arr.wire_targets[e],
            arr.wire_targets[f],
            arr.wire_targets[g],
        ].iter().collect());

        arr.segment_patterns.push([ // 7
            arr.wire_targets[a],
            arr.wire_targets[c],
            arr.wire_targets[f],
        ].iter().collect());

        arr.segment_patterns.push([ // 8
            arr.wire_targets[a],
            arr.wire_targets[b],
            arr.wire_targets[c],
            arr.wire_targets[d],
            arr.wire_targets[e],
            arr.wire_targets[f],
            arr.wire_targets[g],
        ].iter().collect());

        arr.segment_patterns.push([ // 9
            arr.wire_targets[a],
            arr.wire_targets[b],
            arr.wire_targets[c],
            arr.wire_targets[d],
            arr.wire_targets[f],
            arr.wire_targets[g],
        ].iter().collect());

        arr.normalized_id = normalized_value(&arr.segment_patterns);

        return arr;
    }

    // Decodes a single signal into the numeric value being communicated.
    fn decode_one(&self, signal: &str) -> i32 {
        for (i, check_signal) in self.segment_patterns.iter().enumerate() {
            if normalized_byte(signal) == normalized_byte(&check_signal) {
                return i as i32;
            }
        }
        panic!("Arrangement {:?} cannot decode {}", self.wire_targets, signal);
    }

    // Decodes a series of signals to build up the intended N-digit value.
    fn decode_many(&self, signals: &Vec<String>) -> i32 {
        let mut res = 0;
        for signal in signals {
            res = (10 * res) + self.decode_one(signal);
        }
        return res;
    }
}

// Transforms a signal into a single byte describing that signal.
// The output value is shared by all arrangements of the same signal.
// For instance, "abcg", "gcab", "agbc", etc. are all just different
// arrangements of the same signal, so they have the same normalized
// byte value.
fn normalized_byte(signal: &str) -> u8 {
    let mut byte = 0u8;
    for ch in signal.chars() {
        byte = byte | match ch {
            'a' => 1,
            'b' => 2,
            'c' => 4,
            'd' => 8,
            'e' => 16,
            'f' => 32,
            'g' => 64,
            _ => panic!("Invalid segment in signal: {}", ch)
        };
    }
    return byte;
}

// Transforms a set of signals into a unique value describing that
// set of signals. Each signal in the list is first normalized. Then,
// the normalized values are sorted and joined together into an 80-bit
// binary value optimized for comparisions. This lets us detect if
// different sets of signals are actually just re-arrangements of the
// same canonical set.
fn normalized_value(signals: &Vec<String>) -> u128 {
    let mut normalized_bytes: Vec<u8> = signals.iter()
        .map(|s| { s.as_str() })
        .map(normalized_byte)
        .collect();
    normalized_bytes.sort();

    let mut value = 0u128;
    let mut offset: usize = 0;
    for &byte in normalized_bytes.iter().take(10) {
        value = value | ((byte as u128) << offset);
        offset += 8;
    }

    return value;
}

fn main() {
    // Read all entries from input
    let mut entries: Vec<Entry> = Vec::new();
    while let Some(entry) = read_entry() {
        entries.push(entry);
    }

    // Part 1 - Answer: 349
    let mut obvious_output_digits_count = 0;
    for entry in entries.iter() {
        obvious_output_digits_count += entry.outputs.iter()
            .map(|s| { s.len() })
            .filter(|&n| { n == 2usize || n == 3usize || n == 4usize || n == 7usize })
            .count();
    }
    println!("Total 'obvious' outputs: {}", obvious_output_digits_count);

    // Part 2 - Answer: 1070957
    // For every possible arrangement, calculate the resulting signal
    // patterns. Store them in a map to quickly lookup the arrangement
    // given the unique normalized value.
    println!("Computing lookup table");
    let mut arrangements: BTreeMap<u128, Arrangement> = BTreeMap::new();
    let wire_targets: Vec<char> = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g'];
    for perm in wire_targets.into_iter().permutations(7) {
        let arrangement = Arrangement::new(perm);
        arrangements.insert(arrangement.normalized_id, arrangement);
    }
    println!("Computing lookup table - done");

    // For every entry in the input, compute it's normalized value and find
    // it in the pre-computed arrangements.
    let mut decoded_sum = 0;
    for entry in entries.iter() {
        let entry_id = normalized_value(&entry.signal_patterns);
        if let Some(arr) = arrangements.get(&entry_id) {
            let decoded_value = arr.decode_many(&entry.outputs);
            decoded_sum += decoded_value;
        }
        else {
            panic!("Did not find any arrangement for entry {:?} ({})", entry.signal_patterns, entry_id);
        }
    }
    println!("Decoded sum: {}", decoded_sum);
}