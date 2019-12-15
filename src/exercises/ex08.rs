use crate::helpers::file::{read, write};

pub fn ex08() {
    let e = "08";
    let s = read(e);
    write(
        e,
        &sub1(&s, 25, 6).to_string(),
        &sub2(&s, 25, 6).to_string(),
    );
}

pub fn sub1(s: &str, width: i32, height: i32) -> i32 {
    let sip = s
        .chars()
        .map(|x| x.to_digit(10).unwrap_or(0) as i32)
        .collect::<Vec<i32>>();
    let image = Image::new(sip, width, height);
    mult_12(image)
}

pub fn sub2(s: &str, width: i32, height: i32) -> String {
    let sip = s
        .chars()
        .map(|x| x.to_digit(10).unwrap_or(0) as i32)
        .collect::<Vec<i32>>();
    let image = Image::new(sip, width, height);
    result(image)
}

fn result(image: Image) -> String {
    let mut res = "".to_string();
    for i in 0..image.height {
        for j in 0..image.width {
            let pixels = image
                .layers
                .iter()
                .map(|l| l[(i * image.width + j) as usize])
                .collect::<Vec<i32>>();
            res.push_str(&format!("{} ", color(pixels)));
        }
        res.push_str("\n");
    }
    res
}

fn color(pixels: Vec<i32>) -> i32 {
    let mut c = 2;
    for i in pixels.iter() {
        if *i == 0 || *i == 1 {
            c = *i;
            break;
        }
    }
    c
}

fn mult_12(image: Image) -> i32 {
    let mut index0 = 0;
    let mut min0 = image.height * image.width;
    for (i, layer) in image.layers.iter().enumerate() {
        let c = layer
            .iter()
            .map(|p| if *p == 0 { 1 } else { 0 })
            .sum::<i32>();
        if c < min0 {
            min0 = c;
            index0 = i;
        }
    }
    image.layers[index0]
        .iter()
        .map(|p| if *p == 1 { 1 } else { 0 })
        .sum::<i32>()
        * image.layers[index0]
            .iter()
            .map(|p| if *p == 2 { 1 } else { 0 })
            .sum::<i32>()
}

#[derive(Debug)]
struct Image {
    width: i32,
    height: i32,
    layers: Vec<Vec<i32>>,
}

impl Image {
    fn new(v: Vec<i32>, width: i32, height: i32) -> Self {
        let mut layers = vec![];
        let mut i = 0;
        while i < v.len() {
            let mut j = 0;
            let mut layer = vec![];
            while j < height * width {
                layer.push(v[i as usize]);
                j += 1;
                i += 1;
            }
            layers.push(layer);
        }
        Self {
            layers,
            width,
            height,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sub11() {
        assert_eq!(sub1("123456789012", 3, 2), 1);
    }

    #[test]
    fn sub12() {
        assert_eq!(sub1("112456789012", 3, 2), 2);
    }

    #[test]
    fn sub21() {
        assert_eq!(sub2("0222112222120000", 2, 2), "0 1 \n1 0 \n");
    }

    #[test]
    fn sub22() {
        assert_eq!(sub2("022210202100121120", 3, 2), "0 0 1 \n1 1 0 \n");
    }
}
