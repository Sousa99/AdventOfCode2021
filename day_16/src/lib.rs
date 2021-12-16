use std::fmt;

// ================================================== STRUCTS ==================================================

type ID = u64;

#[derive(Copy, Clone)]
enum PacketTypeEnum {
    Sum,
    Product,
    Minimum,
    Maximum,
    Literal,
    GreaterThan,
    LessThan,
    EqualTo,
}

struct PacketType {
    id:                 ID,
    packet_type_enum:   PacketTypeEnum,
}

pub struct Packet {
    version_id:     ID,
    packet_type:    PacketType,
    // Possibilities
    literal:        Option<u64>,
    sub_packets:    Option<Vec<Packet>>,
}

pub struct SystemBITS {
    packets:        Vec<Packet>,
}

// ================================================== AUX FUNCTIONS ==================================================

fn all_zeros(transmission_bin: &String) -> bool {
    return !transmission_bin.chars().any(|characther| characther == '1');
}

fn convert_hex_binary(hex_char: char) -> String {
    match hex_char {
        '0' => String::from("0000"),
        '1' => String::from("0001"),
        '2' => String::from("0010"),
        '3' => String::from("0011"),
        '4' => String::from("0100"),
        '5' => String::from("0101"),
        '6' => String::from("0110"),
        '7' => String::from("0111"),
        '8' => String::from("1000"),
        '9' => String::from("1001"),
        'A' => String::from("1010"),
        'B' => String::from("1011"),
        'C' => String::from("1100"),
        'D' => String::from("1101"),
        'E' => String::from("1110"),
        'F' => String::from("1111"),
        _ => panic!("🚨  Characther not recognized as hexadecimal: {}", hex_char),
    }
}

fn convert_id_packet_type(packet_type_id: ID) -> PacketType {

    let type_enum : PacketTypeEnum = match packet_type_id {
        0 =>    PacketTypeEnum::Sum,
        1 =>    PacketTypeEnum::Product,
        2 =>    PacketTypeEnum::Minimum,
        3 =>    PacketTypeEnum::Maximum,
        4 =>    PacketTypeEnum::Literal,
        5 =>    PacketTypeEnum::GreaterThan,
        6 =>    PacketTypeEnum::LessThan,
        7 =>    PacketTypeEnum::EqualTo,
        _ =>    panic!("🚨  No known PacketType")
    };

    PacketType {
        id:                 packet_type_id,
        packet_type_enum:   type_enum,
    }
}

fn convert_packet_type_string(packet_type: PacketTypeEnum) -> String {
    return match packet_type {
        PacketTypeEnum::Sum => String::from("Operator : Sum"),
        PacketTypeEnum::Product => String::from("Operator : Product"),
        PacketTypeEnum::Minimum => String::from("Operator : Minimum"),
        PacketTypeEnum::Maximum => String::from("Operator : Maximum"),
        PacketTypeEnum::Literal => String::from("Literal"),
        PacketTypeEnum::GreaterThan => String::from("Operator : GreaterThan"),
        PacketTypeEnum::LessThan => String::from("Operator : LessThan"),
        PacketTypeEnum::EqualTo => String::from("Operator : EqualTo"),
    }
}

// ================================================== IMPLEMENTATIONS ==================================================

impl Packet {

    fn get_literal_from_info(info: String) -> (u64, String) {

        const LITERAL_SIZE : usize = 5;

        let info_chunks : Vec<Vec<char>> = info.chars()
            .collect::<Vec<char>>()
            .chunks(LITERAL_SIZE)
            .map(|chunk| chunk.into_iter()
                .collect::<String>().chars()
                .collect::<Vec<char>>())
            .collect();

        let mut remaining_info : String = String::new();
        let mut reached_zero : bool = false;
        let mut literal_binary : String = String::new();
        for info_chunk in info_chunks.into_iter() {

            if reached_zero {
                for characther in info_chunk { remaining_info.push(characther) }
            } else {
                let bit = info_chunk[0];
                for characther in info_chunk { literal_binary.push(characther) }
                if bit == '0' { reached_zero = true; }
            }
            
        }

        return (ID::from_str_radix(&literal_binary, 2).unwrap(), remaining_info)
    }

    fn get_sub_packets_from_info(info: String) -> (Vec<Packet>, String) {

        let mut info_characthers : Vec<char> = info.chars().collect();
        let length_bit = info_characthers[0];
        info_characthers = info_characthers[1..].to_vec();

        let mut remaining_info : String = String::new();
        let mut packets : Vec<Packet> = Vec::new();

        if length_bit == '0' {

            let total_length_bin : String = info_characthers[0..15].to_vec()
                .into_iter().collect::<String>();
            info_characthers = info_characthers[15..].to_vec();
            let total_length = usize::from_str_radix(&total_length_bin, 2).unwrap();

            let mut info_for_packets : String = info_characthers[0..total_length].into_iter().collect();
            remaining_info = info_characthers[total_length..].into_iter().collect();

            while !all_zeros(&info_for_packets) {

                let return_info = Packet::new(info_for_packets);
                packets.push(return_info.0);
                info_for_packets = return_info.1;
            }

        } else if length_bit == '1' {

            let number_packets_bin : String = info_characthers[0..11].to_vec()
                .into_iter().collect::<String>();
            info_characthers = info_characthers[11..].to_vec();
            let number_packets = usize::from_str_radix(&number_packets_bin, 2).unwrap();

            remaining_info = info_characthers.into_iter().collect();
            for _ in 0..number_packets {

                let return_info = Packet::new(remaining_info);
                packets.push(return_info.0);
                remaining_info = return_info.1;
            }
        
        }

        return (packets, remaining_info);
    }

    fn new(packet_info_binary: String) -> (Packet, String) {
        
        let version_id_binary = packet_info_binary[0..3].to_owned();
        let version_id = ID::from_str_radix(&version_id_binary, 2).unwrap();
        
        let packet_type_id_binary = packet_info_binary[3..6].to_owned();
        let packet_type_id = ID::from_str_radix(&packet_type_id_binary, 2).unwrap();
        let packet_type = convert_id_packet_type(packet_type_id);

        let mut literal : Option<u64> = None;
        let mut sub_packets : Option<Vec<Packet>> = None;

        let mut remaining_info : String = packet_info_binary[6..].to_string();
        match packet_type.packet_type_enum {
            PacketTypeEnum::Literal => {
                let return_info = Packet::get_literal_from_info(remaining_info);
                literal = Some(return_info.0);
                remaining_info = return_info.1;
            },

            _ => {
                let return_info = Packet::get_sub_packets_from_info(remaining_info);
                sub_packets = Some(return_info.0);
                remaining_info = return_info.1;
            }
        }
        
        let new_packet = Packet {
            version_id: version_id,
            packet_type: packet_type,
            // Possibilities
            literal: literal,
            sub_packets: sub_packets,
        };

        return (new_packet, remaining_info);
    }

    fn sum_versions(&self) -> ID {

        let mut sum_value : ID = self.version_id;
        if self.sub_packets.is_some() {
            for packet in self.sub_packets.as_ref().unwrap().iter() {
                sum_value = sum_value + packet.sum_versions();
            }
        }

        return sum_value;
    }

    fn compute_value(&self) -> u64 {

        let mut sub_packets_values : Vec<u64> = Vec::new();
        if self.sub_packets.is_some() {
            sub_packets_values = self.sub_packets.as_ref().unwrap().iter()
                .map(|packet| packet.compute_value())
                .collect();
        }

        let final_value : u64 = match self.packet_type.packet_type_enum {

            PacketTypeEnum::Sum => sub_packets_values.iter().sum(),
            PacketTypeEnum::Product => sub_packets_values.iter().product(),
            PacketTypeEnum::Minimum => *sub_packets_values.iter().min().unwrap(),
            PacketTypeEnum::Maximum => *sub_packets_values.iter().max().unwrap(),
            PacketTypeEnum::Literal => self.literal.unwrap(),
            PacketTypeEnum::GreaterThan => if sub_packets_values[0] > sub_packets_values[1] { 1 } else { 0 },
            PacketTypeEnum::LessThan => if sub_packets_values[0] < sub_packets_values[1] { 1 } else { 0 },
            PacketTypeEnum::EqualTo => if sub_packets_values[0] == sub_packets_values[1] { 1 } else { 0 },
        };

        return final_value;

    }

    fn display(&self, ident: usize) -> String {

        let mut line_ident : String = String::new();
        for _ in 0..ident { line_ident = format!("{}\t", line_ident); }

        let mut line : String = format!("{}Version Id: {}", line_ident, self.version_id);
        line = format!("{}\n{}Packet Id: {} ({})", line, line_ident, self.packet_type.id, convert_packet_type_string(self.packet_type.packet_type_enum));
        match self.packet_type.packet_type_enum {
            PacketTypeEnum::Literal => {
                line = format!("{}\n{}Literal: {}", line, line_ident, self.literal.unwrap());
            },

            _ => {
                line = format!("{}\n{}Packets:\n", line, line_ident);
                for sub_packet in self.sub_packets.as_ref().unwrap() {
                    line = format!("{}{}\n", line, sub_packet.display(ident + 1));
                }
            }
        }

        return line;
    }
}

impl fmt::Display for Packet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "{}", self.display(0));
    }
}

impl SystemBITS {

    pub fn new(transmission_hex: String) -> SystemBITS {

        let mut transmission_bin : String = transmission_hex.chars()
            .into_iter()
            .map(|hex_char| convert_hex_binary(hex_char))
            .collect::<Vec<String>>()
            .join("");

        let mut packets : Vec<Packet> = Vec::new();
        while !all_zeros(&transmission_bin) {

            let return_info = Packet::new(transmission_bin);
            packets.push(return_info.0);
            transmission_bin = return_info.1;
        }

        SystemBITS {
            packets: packets
        }
    }

    pub fn get_sum_of_versions_of_packets(&self) -> Vec<ID> {
        return self.packets.iter()
            .map(|packet| packet.sum_versions())
            .collect();
    }

    pub fn compute_values(&self) -> Vec<ID> {
        return self.packets.iter()
            .map(|packet| packet.compute_value())
            .collect();
    }
}

impl fmt::Display for SystemBITS {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        for packet in self.packets.iter() { write!(f, "{}", packet)? }
        return Ok(());
    }
}
