/*
	Defining Enums
		Enums are a way to define a type that can have a fixed set of values.
		Enums are useful when you want to define a type that can have a fixed set of values.
		Enums are also useful when you want to define a type that can have multiple types of values.
		Enums are defined using the enum keyword.
		Enums can be defined with a set of named variants, or unnamed variants.
		Named variants are variants that have a name associated with them.
		Unnamed variants are variants that don't have a name associated with them.
		Enums can be defined with a set of variants, or a single variant.
		Enums can be defined with a set of variants, or a single variant.

		Variants
			Variants are the different values that an enum can have.
			Variants are defined using the enum keyword.
			Enums can be defined with a set of named variants, or unnamed variants.
			Named variants are variants that have a name associated with them.
			Unnamed variants are variants that don't have a name associated with them.
			Enums can be defined with a set of variants, or a single variant.
			Enums can be defined with a set of variants, or a single variant.
*/

fn main() {
	let x = Color::Red;  // x is of type Color

	let y = Color::Blue;  // y is of type Color

	let z = Color::Green;  // z is of type Color

	println!("{:?}", x);  // Red
	println!("{:?}", y);  // Blue
	println!("{:?}", z);  // Green

	let x = Size::Small;  // x is of type Size

	let y = Size::Medium;  // y is of type Size

	let z = Size::Large;  // z is of type Size

	println!("{:?}", x);  // Small
	println!("{:?}", y);  // Medium
	println!("{:?}", z);  // Large

	let x = Message::Quit;  // x is of type Message

	let y = Message::Move { x: 5, y: 10 };  // y is of type Message

	if let Message::Move { x, y } = x {
		println!("Moving to coordinates: x: {}, y: {}", x, y);
	}

	let z = Message::Write(String::from("Hello"));  // z is of type Message

	let a = Message::ChangeColor(Color::Red);  // a is of type Message

	let b = Message::ChangeSize(Size::Small);  // b is of type Message

	println!("{:?}", x);  // Quit
	println!("{:?}", y);  // Move { x: 5, y: 10 }
	println!("{:?}", z);  // Write("Hello")
	println!("{:?}", a);  // ChangeColor(Red)
	println!("{:?}", b);  // ChangeSize(Small)

}

#[derive(Debug)]
enum Color {
	Red,
	Blue,
	Green,
}

#[derive(Debug)]
enum Size {
	Small,
	Medium,
	Large,
}

#[derive(Debug)]
#[allow(dead_code)]
enum Message {
	Quit,
	Move { x: i32, y: i32 },
	Write(String),
	ChangeColor(Color),
	ChangeSize(Size),
}