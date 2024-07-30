use core::fmt;

pub fn example_struct() {
  struct User {
    name: String,
    email: String,
    active: bool,
    age: u8
  }

  let user = User {
    name: String::from("John"),
    email: String::from("johndoe@example.com"),
    active: true,
    age: 24
  };

  println!("user => {} {}", user.name, user.email);
  println!("user => {} {}", user.active, user.age);
}

pub fn example_enum() {
  enum WeekDays {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday
  }

  impl fmt::Display for WeekDays {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
      match self {
        WeekDays::Monday => write!(f, "Monday"),
        WeekDays::Tuesday => write!(f, "Tuesday"),
        WeekDays::Wednesday => write!(f, "Wednesday"),
        WeekDays::Thursday => write!(f, "Thursday"),
        WeekDays::Friday => write!(f, "Friday"),
        WeekDays::Saturday => write!(f, "Saturday"),
        WeekDays::Sunday => write!(f, "Sunday"),
      }
    }
  }

  println!("day => {}", WeekDays::Sunday);
  println!("day => {}", WeekDays::Monday);
  println!("day => {}", WeekDays::Tuesday);
  println!("day => {}", WeekDays::Wednesday);
  println!("day => {}", WeekDays::Thursday);
  println!("day => {}", WeekDays::Friday);
  println!("day => {}", WeekDays::Saturday);
}