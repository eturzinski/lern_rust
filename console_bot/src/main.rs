fn main() {

  //initState
  let mut dialogState = DialogState::INIT(StateData {text:String::from("HelloWorld!")});

  println!("{}",dialogState.stateData.text)
}

enum DialogState{
  INIT(StateData),
}


struct StateData{
  text:String
}

