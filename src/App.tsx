import React, {useEffect} from 'react';
import './App.css';
import init from "./pkg/wasm_lib";
import MiddleSection from "./components/middle-section/MiddleSection";

function App() {
  useEffect(() => {
    init().then(_ => { /* do nothing */ });
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
