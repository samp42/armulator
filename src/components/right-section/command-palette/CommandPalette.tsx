import {Button, Card} from "react-bootstrap";
import './CommandPalette.css';

function CommandPalette() {

    return (
        <div className="command-palette">
            <Card className="command-palette-card">
                <Card.Body className="command-palette-card-body">
                    <Button className="command-palette-btn" id="pause-btn" onClick={() => {console.log("pause")}}>
                        <img className="command-palette-btn-img" id="pause-btn-img" src="pause-circle.svg" alt="Pause" width="20"></img>
                        <img className="command-palette-btn-img-hover" id="pause-btn-img-hover" src="pause-circle-fill.svg" alt="Pause"
                             width="20"></img>
                    </Button>
                    <Button className="command-palette-btn" id="run-btn" onClick={() => {console.log("run")}}>
                        <img className="command-palette-btn-img" id="run-btn-img" src="play-circle.svg" alt="Run" width="20"></img>
                        <img className="command-palette-btn-img-hover" id="run-btn-img-hover" src="play-circle-fill.svg" alt="Run"
                             width="20"></img>
                    </Button>
                    <Button className="command-palette-btn" id="stop-btn" onClick={() => {console.log("stop")}}>
                        <img className="command-palette-btn-img" id="stop-btn-img" src="stop-circle.svg" alt="Stop" width="20"></img>
                        <img className="command-palette-btn-img-hover" id="stop-btn-img-hover" src="stop-circle-fill.svg" alt="Stop"
                             width="20"></img>
                    </Button>
                </Card.Body>
            </Card>
        </div>
    );
}

export default CommandPalette;
