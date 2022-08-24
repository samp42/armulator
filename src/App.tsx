import React, {useEffect} from 'react';
import './App.css';
import init, {add, parse} from "wasm-lib";
import MiddleSection from "./components/middle-section/MiddleSection";

function App() {
  useEffect(() => {
    init();
  }, []);
  return (
      <div className="App">
        {/*<LeftSection></LeftSection>*/}
        <MiddleSection></MiddleSection>
        {/*<RightSection></RightSection>*/}
      </div>
  );
}

export default App;
