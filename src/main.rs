struct Project {
    price: f32,
}

struct MaintenanceHours {
    rate: f32,
    hours: f32,
}

trait Billable {
    fn bill(&self) -> f32;
}

impl Billable for Project {
    fn bill(&self) -> f32 {
        self.price
    }
}

impl Billable for MaintenanceHours {
    fn bill(&self) -> f32 {
        self.rate * self.hours
    }
}

impl Billable for f32 {
    fn bill(&self) -> f32 {
        self.clone()
    }
}

impl Into<Project> for f32 {
    fn into(self) -> Project {
        Project { price: self }
    }
}

impl Into<f32> for Project {
    fn into(self) -> f32 {
        self.price
    }
}

fn print_billable(billable: &impl Billable) {
    println!("{}", billable.bill());
}

fn print_billables<T: Billable, U: Billable>(bill: &T, gates: &U) {
    println!("{}, {}", bill.bill(), gates.bill());
}

fn print_project(proj: &Project) {
    println!("{}", proj.bill());
}

fn main() {
    let proj = Project { price: 5000.0 };
    let proj_2 = Project { price: 5000.0 };
    print_project(&5000.0.into());

    let maintenance_hours = MaintenanceHours {
        hours: 200.0,
        rate: 80.0,
    };
    print_billable(&proj);
    print_billable(&maintenance_hours);
    print_billable(&1000.0);
    print_billables(&proj, &maintenance_hours);
    print_billables(&proj, &proj_2);
}
