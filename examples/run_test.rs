use std::process::Command;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    // run_example("neure_nocap", "regex_nocap", 300_0000)?;
    // run_example("neure_cap", "regex_cap", 300_0000)?;
    run_example("neure_cap", "nom_cap", 1000_0000)?;
    Ok(())
}

fn run_example(name: &str, example: &str, check: usize) -> Result<(), Box<dyn std::error::Error>> {
    let rel_out = run_and_read_result(example, TestType::Rel, 10)?;
    let fat_out = run_and_read_result(example, TestType::Fat, 10)?;

    assert_eq!(rel_out.0.count, check);
    assert_eq!(rel_out.1.count, check);
    assert_eq!(fat_out.0.count, check);
    assert_eq!(fat_out.1.count, check);
    println!(
        "| {}/{} | {} | {}ms | {:.4}mu |",
        name, "rel", rel_out.0.size, rel_out.0.cost, rel_out.0.avg
    );
    println!(
        "| {}/{} | {} | {}ms | {:.4}mu |",
        example, "rel", rel_out.1.size, rel_out.1.cost, rel_out.1.avg
    );
    println!(
        "| {}/{} | {} | {}ms | {:.4}mu |",
        name, "fat", fat_out.0.size, fat_out.0.cost, fat_out.0.avg
    );
    println!(
        "| {}/{} | {} | {}ms | {:.4}mu |",
        example, "fat", fat_out.1.size, fat_out.1.cost, fat_out.1.avg
    );

    Ok(())
}

fn run_and_read_result(
    example: &str,
    ty: TestType,
    count: usize,
) -> Result<(Res, Res), Box<dyn std::error::Error>> {
    let mut res = (Res::default(), Res::default());

    cargo_build(example, ty)?;
    for _ in 0..count {
        let output = cargo_run(example, ty)?;
        let lines = output.lines().collect::<Vec<&str>>();

        res.0.merge(&Res::from_str(lines[0])?);
        res.1.merge(&Res::from_str(lines[1])?);
    }
    res.0.avg(count);
    res.1.avg(count);
    Ok(res)
}

fn cargo_build(example: &str, ty: TestType) -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::new("cargo");

    cmd.arg("build");
    cmd.arg("-j8");
    cmd.arg("-q");
    cmd.arg(format!("--example={}", example));
    ty.add_arg(&mut cmd);

    cmd.spawn()?.wait()?;
    Ok(())
}

fn cargo_run(example: &str, ty: TestType) -> Result<String, Box<dyn std::error::Error>> {
    let mut cmd = Command::new("cargo");

    cmd.arg("run");
    cmd.arg("-q");
    cmd.arg(format!("--example={}", example));
    ty.add_arg(&mut cmd);

    let output = cmd.output()?;
    let stdout = output.stdout;

    Ok(String::from_utf8(stdout)?)
}

#[derive(Debug, Clone, Copy)]
enum TestType {
    Rel,

    Fat,
}

impl TestType {
    pub fn add_arg(&self, cmd: &mut Command) {
        match self {
            TestType::Rel => {
                cmd.arg("--release");
            }
            TestType::Fat => {
                cmd.arg("--profile=release-lto");
            }
        }
    }
}

#[derive(Debug, Default)]
pub struct Res {
    size: usize,
    cost: usize,
    test: usize,
    avg: f64,
    count: usize,
}

impl Res {
    pub fn from_str(str: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let splited = str.split(",");
        let mut ret = Self::default();

        for split in splited {
            if let Some((name, value)) = split.split_once("=") {
                let name = name.trim();
                let value = value.trim();

                match name {
                    "Size" => {
                        ret.size = value.parse::<usize>()?;
                    }
                    "Cost" => {
                        ret.cost = value.parse::<usize>()?;
                    }
                    "Test" => {
                        ret.test = value.parse::<usize>()?;
                    }
                    "Avg" => {
                        ret.avg = value.parse::<f64>()?;
                    }
                    "Count" => {
                        ret.count = value.parse::<usize>()?;
                    }
                    _ => {}
                }
            }
        }

        Ok(ret)
    }

    pub fn merge(&mut self, other: &Self) {
        self.size += other.size;
        self.cost += other.cost;
        self.test += other.test;
        self.avg += other.avg;
        self.count += other.count;
    }

    pub fn avg(&mut self, count: usize) {
        self.size /= count;
        self.cost /= count;
        self.test /= count;
        self.avg /= count as f64;
        self.count /= count;
    }
}
