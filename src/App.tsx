import React, {useEffect} from 'react';
import './App.css';
import MiddleSection from "./components/middle-section/MiddleSection";
import init from "./pkg";

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
