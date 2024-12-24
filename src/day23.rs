use ahash::{AHashMap, AHashSet};

fn parse(input: &str) -> (Vec<AHashSet<usize>>, AHashSet<usize>, Vec<String>) {
    let pairs = input
        .lines()
        .map(|line| line.trim())
        .filter(|&line| !line.is_empty())
        .map(|line| line.split("-").collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let vertices = pairs
        .iter()
        .flatten()
        .fold(AHashSet::new(), |mut acc, x| {
            acc.insert(x.to_string());
            acc
        })
        .into_iter()
        .collect::<Vec<String>>();
    let vertices_ids = vertices
        .iter()
        .enumerate()
        .map(|(i, v)| (v.clone(), i))
        .collect::<AHashMap<_, _>>();
    let mut edges = vec![AHashSet::new(); vertices.len()];
    pairs.into_iter().for_each(|pair| {
        let s = vertices_ids[pair[0]];
        let d = vertices_ids[pair[1]];
        edges[s].insert(d);
        edges[d].insert(s);
    });
    let special = vertices_ids
        .into_iter()
        .filter(|(key, _)| key.as_bytes()[0] == b't')
        .map(|(_, value)| value)
        .collect::<AHashSet<_>>();
    (edges, special, vertices)
}

fn bron_kerbosch(
    edges: &Vec<AHashSet<usize>>,
    r: AHashSet<usize>,
    mut p: AHashSet<usize>,
    mut x: AHashSet<usize>,
    cliques: &mut Vec<AHashSet<usize>>,
) {
    if p.is_empty() && x.is_empty() {
        cliques.push(r.clone());
    } else {
        while !p.is_empty() {
            let v = *p.iter().next().unwrap();
            let mut new_r = r.clone();
            let mut new_p = AHashSet::new();
            let mut new_x = AHashSet::new();
            new_r.insert(v);
            for &n in edges[v].iter() {
                if p.contains(&n) {
                    new_p.insert(n);
                }
                if x.contains(&n) {
                    new_x.insert(n);
                }
            }
            bron_kerbosch(edges, new_r, new_p, new_x, cliques);
            p.remove(&v);
            x.insert(v);
        }
    }
}

pub fn part1(input: &str) -> i64 {
    let (edges, special, _) = parse(input);
    let mut count = [0; 3];
    for &v in &special {
        for i in 0..edges.len() {
            if !edges[v].contains(&i) {
                continue;
            }
            for j in i + 1..edges.len() {
                if edges[v].contains(&j) && edges[i].contains(&j) {
                    let mut index = 0;
                    if special.contains(&i) {
                        index += 1;
                    }
                    if special.contains(&j) {
                        index += 1;
                    }
                    count[index] += 1;
                }
            }
        }
    }
    count[0] + count[1] / 2 + count[2] / 3
}

pub fn part2(input: &str) -> String {
    let (edges, _, vertices) = parse(input);
    let r = AHashSet::new();
    let p = (0..edges.len()).collect::<AHashSet<_>>();
    let x = AHashSet::new();
    let mut cliques = vec![];
    bron_kerbosch(&edges, r, p, x, &mut cliques);
    let clique = cliques
        .iter()
        .map(|x| (x.len(), x))
        .max_by_key(|x| x.0)
        .unwrap()
        .1;
    let mut names = clique
        .iter()
        .map(|&v| vertices[v].clone())
        .collect::<Vec<_>>();
    names.sort_unstable();
    names.join(",")
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
        kh-tc
        qp-kh
        de-cg
        ka-co
        yn-aq
        qp-ub
        cg-tb
        vc-aq
        tb-ka
        wh-tc
        yn-cg
        kh-ub
        ta-co
        de-co
        tc-td
        tb-wq
        wh-td
        ta-ka
        td-qp
        aq-cg
        wq-ub
        ub-vc
        de-ta
        wq-aq
        wq-vc
        wh-yn
        ka-de
        kh-ta
        co-tc
        wh-qp
        tb-vc
        td-yn
        ";

    #[test]
    fn test_day22_part1() {
        assert_eq!(part1(INPUT), 7);
    }

    #[test]
    fn test_day22_part2() {
        assert_eq!(part2(INPUT), "co,de,ka,ta");
    }
}
