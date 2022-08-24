import {Card} from "react-bootstrap";
import "./Diodes.css";
import {DiodeArray} from "../../../models/DiodeArray";

function Diodes(props: DiodeArray) {

    function getImgSrc(index: number): string {
        return props.diodes[index] ? "circle-fill.svg" : "circle.svg";
    }

    return (
        <div className="diodes">
            <Card className="diodes-card">
                <Card.Header className="diodes-card-header">Diodes</Card.Header>
                <Card.Body className="diodes-card-body">
                    <img className="diode-img" src={getImgSrc(0)} alt="diode-0"></img>
                    <img className="diode-img" src={getImgSrc(1)} alt="diode-1"></img>
                    <img className="diode-img" src={getImgSrc(2)} alt="diode-2"></img>
                    <img className="diode-img" src={getImgSrc(3)} alt="diode-3"></img>
                    <img className="diode-img" src={getImgSrc(4)} alt="diode-4"></img>
                    <img className="diode-img" src={getImgSrc(5)} alt="diode-5"></img>
                    <img className="diode-img" src={getImgSrc(6)} alt="diode-6"></img>
                    <img className="diode-img" src={getImgSrc(7)} alt="diode-7"></img>
                </Card.Body>
            </Card>
        </div>
    );
}

export default Diodes;
