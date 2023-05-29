// First, we are going to introduce some units of energy. For whatever reason, we prefer BTU above
// Joules and Calories, but we want to support all 3 of these in this module. Double check the
// conversion methods, and make sure you fully understand them.

use std::marker::PhantomData;

// You may uncomment and use the following import if you need it. You may also read its
// documentation at https://doc.rust-lang.org/std/cell/struct.RefCell
use std::cell::RefCell;

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub struct Joule(u32);
#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub struct Calorie(u32);

pub type BTU = u32;

impl From<Joule> for BTU {
	fn from(j: Joule) -> Self {
		j.0 / 1055
	}
}

impl From<BTU> for Joule {
	fn from(b: BTU) -> Self {
		Self(b * 1055)
	}
}

impl From<Calorie> for BTU {
	fn from(c: Calorie) -> Self {
		c.0 / 251
	}
}

impl From<BTU> for Calorie {
	fn from(b: BTU) -> Self {
		Calorie(b * 251)
	}
}

// Now, we start defining some types of fuel.

/// A technology for storing energy for later consumption.
pub trait Fuel {
	/// The output unit of the energy density.
	///
	/// Think about this: why did we chose this to be an associated type rather than a generic?
	type Output: Into<BTU> + From<BTU>;

	/// The amount of energy contained in a single unit of fuel.
	fn energy_density() -> Self::Output;
}

pub struct Diesel;
impl Fuel for Diesel {
	type Output = Joule;
	fn energy_density() -> Self::Output {
		//todo!("100 BTU")
        100.into()
	}
}

pub struct LithiumBattery;
impl Fuel for LithiumBattery {
	type Output = Calorie;
	fn energy_density() -> Self::Output {
		//todo!("200 BTU")
        200.into()
	}
}

pub struct Uranium;
impl Fuel for Uranium {
	type Output = Joule;
	fn energy_density() -> Self::Output {
		//todo!("1000 BTU")
        1000.into()
	}
}

/// A container for any fuel type.
pub struct FuelContainer<F: Fuel> {
	/// the amount of fuel.
	amount: u32,
	/// Note: Why do we need this? Fuel doesn't really have any methods that require `&self` on it,
	/// so any information that we can get, we can get from `F` as **TYPE**, we don't really need
	/// to store an instance of `F`, like `fuel: F` as a struct field. But to satisfy the compiler,
	/// we muse use `F` somewhere. This is the perfect use case of `PhantomData`.
	_marker: PhantomData<F>,
}

impl<F: Fuel> FuelContainer<F> {
	pub fn new(amount: u32) -> Self {
		Self { amount, _marker: Default::default() }
	}
}

/// Something that can provide energy from a given `F` fuel type, like a power-plant.
pub trait ProvideEnergy<F: Fuel> {
	/// consume the fuel container and return the created energy, based on the power density of the
	/// fuel and potentially other factors.
	///
	/// Some fuel providers might have some kind of decay or inefficiency, which should be reflected
	/// here. Otherwise, [`provide_energy_with_efficiency`] or [`provide_energy_ideal`] might be
	/// good enough.
	///
	/// Not all `ProvideEnergy` implementations need to have internal state. Therefore, this
	/// interface accepts `&self`, not `&mut self`. You might need to use special language features
	/// to overcome this.
	fn provide_energy(&self, f: FuelContainer<F>) -> <F as Fuel>::Output;

	/// Convert the amount of fuel in `f` with an exact efficiency of `e`.
	///
	/// NOTE: all efficiencies are interpreted as u8 values that can be at most 100, and represent a
	/// percent.
	///
	/// This method must be provided as it will be the same in all implementations.
	fn provide_energy_with_efficiency(&self, f: FuelContainer<F>, e: u8) -> <F as Fuel>::Output {	
		(F::energy_density().into() * f.amount * (e as u32) / 100).into()
	}

	/// Same as above, but with an efficiency of 100.
	///
	/// This method must be provided as it will be the same in all implementations.
	fn provide_energy_ideal(&self, f: FuelContainer<F>) -> <F as Fuel>::Output {
		ProvideEnergy::provide_energy_with_efficiency(self,f,100)
	}
}

/// A nuclear reactor that can only consume `Uranium` and provide energy with 99% efficiency.
pub struct NuclearReactor;
// TODO: uncomment and complete:
impl ProvideEnergy<Uranium> for NuclearReactor {
    fn provide_energy(&self, f: FuelContainer<Uranium>) -> <Uranium as Fuel>::Output {
		ProvideEnergy::provide_energy_with_efficiency(self,f,99)
    }
}

/// A combustion engine that can only consume `Diesel`.
///
/// The `DECAY` const must be interpreted as such: per every `DECAY` times `provide_energy` is
/// called on an instance of this type, the efficiency should reduce by one. The initial efficiency
/// must be configurable with a `fn new(efficiency: u8) -> Self`.
/// pub struct InternalCombustion<const DECAY: u32>(/* Fill the fields as needed */);
pub struct InternalCombustion<const DECAY: u32>{
	count: RefCell<u32>,
    current_efficiency: RefCell<u8>
}

impl<const DECAY: u32> InternalCombustion<DECAY> {
	pub fn new(efficiency: u8) -> Self {
		Self {
			count: RefCell::new(DECAY),
            current_efficiency: if efficiency > 100 {
					RefCell::new(100) 
				} else { 
					RefCell::new(efficiency)},
        }
	}
}

// TODO: uncomment and complete:
impl<const DECAY: u32> ProvideEnergy<Diesel> for InternalCombustion<DECAY> {
    fn provide_energy(&self, f: FuelContainer<Diesel>) -> <Diesel as Fuel>::Output{
		let mut count_borrowed = self.count.borrow_mut();
		let mut current_efficiency_borrowed = self.current_efficiency.borrow_mut();
		if *count_borrowed == 0 {
			*count_borrowed = DECAY;
			*current_efficiency_borrowed -= 1;
		}
		*count_borrowed -= 1;
		ProvideEnergy::provide_energy_with_efficiency(self,f,*current_efficiency_borrowed)
    }
}

/// A hypothetical device that can, unlike the `InternalCombustion`, consume **any fuel** that's of
/// type `trait Fuel`. It can provide a fixed efficiency regardless of fuel type. As before,
/// EFFICIENCY is a u8 whose value should not exceed 100 and is interpreted as a percent.
pub struct OmniGenerator<const EFFICIENCY: u8>;

// NOTE: implement `ProvideEnergy` for `OmniGenerator` using only one `impl` block.
// TODO: uncomment and complete:

impl<const EFFICIENCY: u8, F: Fuel> ProvideEnergy<F> for OmniGenerator<EFFICIENCY> {
    fn provide_energy(&self, f: FuelContainer<F>) -> <F as Fuel>::Output{
        ProvideEnergy::provide_energy_with_efficiency(self,f,EFFICIENCY)
    }
}

/// A type that can wrap two different fuel types and mix them together.
///
/// The energy density of the new fuel type is the average of the two given, once converted to BTU.
/// The output unit should also be BTU.
///
/// This can represent a new fuel type, thus it must implement `Fuel`.
pub struct Mixed<F1: Fuel, F2: Fuel>(PhantomData<(F1, F2)>);
// TODO: uncomment and complete:

impl<F1: Fuel, F2: Fuel> Fuel for Mixed<F1, F2> {
    type Output = BTU;

    fn energy_density() -> Self::Output {
		(F1::energy_density().into() + F2::energy_density().into())/2
    }
}

// Now think about how you can make the mixer configurable, such that it would produce a new fuel
// with an energy density that is more influences by one type than the other.
//
// For example, you have a mixer of F1, F2, and some coefficient C1, where the energy density of the
// mixture is `F1 * C1 + F2 * (1 - C1) )` where `C1` is a ratio (which you have to represent again
// with a u8 percent).
//
// The main trick is to overcome the fact that `fn energy_density` does not take in a `self`, so the
// coefficients need to be incorporated in some other way (you've already seen examples of that in
// this file ;)).
pub struct CustomMixed<const C: u8, F1, F2>(PhantomData<(F1, F2)>);
// TODO: uncomment and complete:
impl<const C: u8, F1: Fuel, F2: Fuel> Fuel for CustomMixed<C, F1, F2> {
    type Output = BTU;

    fn energy_density() -> Self::Output {
		F1::energy_density().into() * (C as u32)/100 + F2::energy_density().into() - F2::energy_density().into() * (C as u32)/100
    }
}

// Now, any of our existing energy providers can be used with a mix fuel.
/// A function that returns the energy produced by the `OmniGenerator` with efficiency of 80%, when
/// the fuel type is a mix of `Diesel` as `LithiumBattery`;
pub fn omni_80_energy(amount: u32) -> BTU {
	let fuel = FuelContainer::<Mixed::<Diesel,LithiumBattery>>::new(amount);
	OmniGenerator::<80>.provide_energy(fuel)
}

// Finally, let's consider marker traits, and some trait bounds.

/// Some traits are just markers. They don't bring any additional functionality anything, other than
/// marking a type with some trait.
pub trait IsRenewable {}
impl IsRenewable for LithiumBattery {}

/// Define the following struct such that it only provides energy if the fuel is `IsRenewable`.
///
/// It has perfect efficiency.
pub struct GreenEngine<F: Fuel>(PhantomData<F>);
// TODO: uncomment and complete:
impl<F: Fuel + IsRenewable> ProvideEnergy<F> for GreenEngine<F> {
	fn provide_energy(&self, f: FuelContainer<F>) -> <F as Fuel>::Output{
        ProvideEnergy::provide_energy_ideal(self,f)
    }
}

/// Define the following struct such that it only provides energy if the fuel's output type is
/// `BTU`.
///
/// It has perfect efficiency.
pub struct BritishEngine<F: Fuel>(PhantomData<F>);
// TODO: uncomment and complete:
impl<F: Fuel> ProvideEnergy<F> for BritishEngine<F> {
	fn provide_energy(&self, f: FuelContainer<F>) -> <F as Fuel>::Output {
		ProvideEnergy::provide_energy_ideal(self,f)
    }
}

// Congratulations! you have finished the advance trait section.
//
// Disclaimer: the types and traits that you are asked to implement in this module are by no means
// designed to be sensible. Instead, they are chosen to represent a typical, often difficult,
// pattern. Some are intentionally slightly convoluted to challenge you :). I am sure if we actually
// wanted to design a fuel system, we would do better.


fn main() {
	/*
    let mut j = Joule(0);
    let btu: BTU;
    let mut c = Calorie(0);

    
    j = 1.into(); //btu to joule
    c = 1.into(); //btu to cal
    //btu = BTU::from(j); //j to btu
    btu = j.into();

    j = Joule(105500);
    c = Calorie(50200);
    let mut u = Joule(1055000);
	*/
	/*
    println!("j[BTU]: {}",BTU::from(j));
    //println!("j[J]: {:?}",Joule::from(btu));
    println!("c[BTU]: {}",BTU::from(c));
    //println!("c[Cal]: {:?}",c);
    println!("c[BTU]: {}",BTU::from(u));

    //let mut diesel: Diesel;

    println!("Diesel[Cal]: {:?}",Diesel::energy_density());
	println!("Diesel[BTU]: {:?}",BTU::from(Diesel::energy_density()));
    println!("LithiumBattery: {:?}",LithiumBattery::energy_density());
    println!("Uranium: {:?}",Uranium::energy_density());

	*/
    //------

    let uranium = FuelContainer::<Uranium>::new(2);

    let nuclear_reactor = NuclearReactor;
	println!("Uranium: {:?}",Uranium::energy_density());
	println!("Uranium: {:?}",Uranium::energy_density());
    println!("Uranium: {:?}",nuclear_reactor.provide_energy(uranium));
	
	let omni = OmniGenerator::<50>;
	let uranium = FuelContainer::<Uranium>::new(1);
	println!("omni: {:?}",OmniGenerator::<50>.provide_energy(uranium));

	println!("Mixed::<Uranium,Diesel>ED: {:?}",Mixed::<Diesel,LithiumBattery>::energy_density());
	println!("CustomMixed::<Uranium,Diesel>ED: {:?}",CustomMixed::<50,Diesel,LithiumBattery>::energy_density());

	println!("omni_80_energy is: {}",omni_80_energy(2));
	///*
	let diesel = FuelContainer::<Diesel>::new(100);
	let ic = InternalCombustion::<3>::new(120);
	println!("IC: {:?}",ic.provide_energy(diesel));
	let diesel = FuelContainer::<Diesel>::new(100);
	println!("IC: {:?}",ic.provide_energy(diesel));
	let diesel = FuelContainer::<Diesel>::new(100);
	println!("IC: {:?}",ic.provide_energy(diesel));
	let diesel = FuelContainer::<Diesel>::new(100);
	println!("IC: {:?}",ic.provide_energy(diesel));
	let diesel = FuelContainer::<Diesel>::new(100);
	println!("IC: {:?}",ic.provide_energy(diesel));

	println!("{}",1055000 * (98 as u32) / 100);
	//*/
	let C:u8 = 50;
	println!("{}",(2*(C as u32)/100))
}

