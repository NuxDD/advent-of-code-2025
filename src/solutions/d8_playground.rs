use super::Problem;

type Node = (u64, u64, u64);
type Circuit = Vec<Node>;

pub struct D8_Playground;

impl Problem for D8_Playground {}

impl D8_Playground {
    fn straight_line_distance(from: &Node, to: &Node) -> f64 {
        let from = (from.0 as f64, from.1 as f64, from.2 as f64);
        let to = (to.0 as f64, to.1 as f64, to.2 as f64);

        let sum: f64 = (from.0 - to.0).powi(2) + (from.1 - to.1).powi(2) + (from.2 - to.2).powi(2);

        sum.sqrt()
    }

    fn build_node(line: &str) -> Node {
        let coords: Vec<u64> = line.split(',').map(|x| x.parse::<u64>().unwrap()).collect();
        (coords[0], coords[1], coords[2])
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn playground_simple_example() {
        let input = "162,817,812\n57,618,57\n906,360,560\n592,479,940\n352,342,300\n466,668,158\n542,29,236\n431,825,988\n739,650,466\n52,470,668\n216,146,977\n819,987,18\n117,168,530\n805,96,715\n346,949,466\n970,615,88\n941,993,340\n862,61,35\n984,92,344\n425,690,689".lines().collect();
        let result = D8_Playground::first_part(&input);
        assert_eq!(result, "40");
    }
}
