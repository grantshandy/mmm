const toggleButton = document.getElementById("toggle");
const clearButton = document.getElementById("clear");
const downloadButton = document.getElementById("download");
const stateText = document.getElementById("state");
const weatherText = document.getElementById("weather");
const csvDiv = document.getElementById("csvData");

var response = await fetch("/state");
var json = await response.json();
var state = json.state;
stateText.innerHTML = "State: " + state;

updateCsv();
setWeather();

downloadButton.onclick = async function () {
    const anchor = document.createElement("a");
    anchor.href = "/data.csv";
    anchor.download = "sprinkler.csv";

    document.body.appendChild(anchor);
    anchor.click();
    document.body.removeChild(anchor);
};

clearButton.onclick = async function () {
    var response = await fetch("/clear");

    updateCsv();
};

toggleButton.onclick = async function () {
    var response = await fetch("/toggle");
    var json = await response.json();

    if (json.error != null) {
        alert("Couldn't toggle the pin! Error: " + json.error);
    };

    var state = json.state;
    stateText.innerHTML = "State: " + state;

    updateCsv();
};

async function updateCsv() {
    var response = await fetch("/data.csv");
    var csvText = await response.text();
    var csvHTML = makeTableHTML(CSVToArray(csvText));
    csvDiv.innerHTML = csvHTML;
}

async function setWeather() {
    var response = await fetch("/weather");
    var weather = await response.text();
    weatherText.innerHTML = weather;
}

function CSVToArray(strData, strDelimiter) {
    strDelimiter = strDelimiter || ",";

    var objPattern = new RegExp("(\\" + strDelimiter + "|\\r?\\n|\\r|^)" + '(?:"([^"]*(?:""[^"]*)*)"|' + '([^"\\' + strDelimiter + "\\r\\n]*))", "gi");

    var arrData = [[]];
    var arrMatches = null;

    while ((arrMatches = objPattern.exec(strData))) {
        var strMatchedDelimiter = arrMatches[1];

        if (strMatchedDelimiter.length && strMatchedDelimiter !== strDelimiter) {
            arrData.push([]);
        }

        var strMatchedValue;

        if (arrMatches[2]) {
            strMatchedValue = arrMatches[2].replace(new RegExp('""', "g"), '"');
        } else {
            strMatchedValue = arrMatches[3];
        }

        arrData[arrData.length - 1].push(strMatchedValue);
    }

    return arrData;
}

function makeTableHTML(myArray) {
    var result = "<table border=1>";

    for (var i = 0; i < myArray.length; i++) {
        if (myArray[i][2] == "On") {
            result += '<tr style="background-color: green">';
            result += "<td>" + myArray[i][0] + "</td>";
            result += "<td>" + myArray[i][1] + "</td>";
            result += "<td>" + myArray[i][2] + "</td>";
            result += "</tr>";
        } else if (myArray[i][2] == "Off") {
            result += '<tr style="background-color: #FE4365">';
            result += "<td>" + myArray[i][0] + "</td>";
            result += "<td>" + myArray[i][1] + "</td>";
            result += "<td>" + myArray[i][2] + "</td>";
            result += "</tr>";
        } else {
            result += "<tr>";
            result += "<td>" + myArray[i][0] + "</td>";
            result += "<td>" + myArray[i][1] + "</td>";
            result += "<td>" + myArray[i][2] + "</td>";
            result += "</tr>";
        }
    }
    result += "</table>";

    return result;
}
