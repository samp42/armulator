import React, {useEffect} from 'react';
import './App.css';
import init, {add} from "wasm-lib";
import LeftSection from "./components/left-section/LeftSection";
import MiddleSection from "./components/middle-section/MiddleSection";
import RightSection from "./components/right-section/RightSection";

function App() {
  useEffect(() => {
    init().then(() => {
      test();
    });
  }, []);

  function test() {
    console.log(add(2, 2));
  }
  return (
      <div className="App">
        <LeftSection></LeftSection>
        <MiddleSection></MiddleSection>
        <RightSection></RightSection>
      </div>
  );
}

export default App;
