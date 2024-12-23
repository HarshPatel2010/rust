mod eachenumholditsowndata;
mod patternmatchenum;
mod test1;
mod whenvalueisabsentuseenumOptionTtodealwiththeabsentvalue;
mod enumexample;
mod matches;
mod flowcontroliflet;
mod whileloop;
mod breakloop;
mod labelwithmultiplenestedloops;
mod matchpattern;
mod ifletpracticerust;
mod patterns;
mod accociatedfunctionexample;
mod associatedfunctionandmethods;
mod methodswithownership;
mod methodexersise;
mod implonenums;
mod generics;
mod constgenerics;
mod traitss;
mod deriveTraits;
mod traitasFunc;
mod returntypeImplTraits;
mod traitBound;
mod dynBox;
mod objsafe;

mod objsafedynamicdispatch;
mod stringgg;



// Fix the errors
enum Number {
    Zero,
    One,
    Two,
}

enum Number1 {
    Zero = 0,
    One,
    Two,
}

// C-like enum
enum Number2 {
    Zero = 0,
    One = 1,
    Two = 2,
}


fn main() {
    // An enum variant can be converted to a integer by `as`
    assert_eq!(Number::One as u8 , Number1::One as u8);
    assert_eq!(Number1::One as u8, Number2::One as u8);
    println!("{}",Number::Zero as u8);
    println!("Success!");

    eachenumholditsowndata::main();
    patternmatchenum::main();
    test1::main();
    whenvalueisabsentuseenumOptionTtodealwiththeabsentvalue::main();
    enumexample::main();
    matches::main();
    flowcontroliflet::main();
    whileloop::main();
    breakloop::main();
    labelwithmultiplenestedloops::main();
    matchpattern::main();
    ifletpracticerust::main();
    patterns::main();
    accociatedfunctionexample::main();
    associatedfunctionandmethods::main();
    methodswithownership::main();
    methodexersise::main();
    implonenums::main();
    generics::main();
    constgenerics::main();
    traitss::main();
    deriveTraits::main();
    traitasFunc::main();
    returntypeImplTraits::main();
    traitBound::main();
    dynBox::main();
    objsafedynamicdispatch::main();
    stringgg::main()



}
