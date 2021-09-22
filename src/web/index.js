var response = await fetch("/state");
var json = await response.json();
var state = json.state;
document.getElementById("state").innerHTML = "State: " + state;

updateCsv();
setWeather();

document.getElementById("minuteInput").value = 5;

(function loop() {
	setTimeout(async function () {
        var response = await fetch("/graph_" +document.getElementById("minuteInput").value+"_"+document.getElementById("mainDiv").offsetWidth+"x500.svg");
        var svgText = await response.text();

        document.getElementById("graph").innerHTML = svgText;
        console.log("updating graph...");
        loop()
	}, 500);
}());

document.getElementById("download").onclick = async function() {
    const anchor = document.createElement("a");
    anchor.href = "/data.csv";
    anchor.download = "sprinkler.csv";

    document.body.appendChild(anchor);
    anchor.click();
    document.body.removeChild(anchor);
};

document.getElementById("clear").onclick = async function() {
    var response = await fetch("/clear");

    updateCsv();
};

document.getElementById("toggle").onclick = async function() {
    var response = await fetch("/toggle");
    var json = await response.json();

    if (json.error != null) {
        alert("Couldn't toggle the pin! Error: " + json.error);
    };

    var state = json.state;
    document.getElementById("state").innerHTML = "State: " + state;

    updateCsv();
};

document.getElementById("refresh").onclick = async function() {
    updateCsv();
    setWeather();
}

async function updateCsv() {
    var response = await fetch("/data.csv");
    var csvText = await response.text();
    var csvHTML = makeTableHTML(CSVToArray(csvText));
    document.getElementById("csvData").innerHTML = csvHTML;
}

async function setWeather() {
    var response = await fetch("/weather");
    var weather = await response.text();
    document.getElementById("weather").innerHTML = weather;
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

function getWidth() {
        return Math.max(
        document.body.scrollWidth,
        document.documentElement.scrollWidth,
        document.body.offsetWidth,
        document.documentElement.offsetWidth,
        document.documentElement.clientWidth
    );
}

function makeTableHTML(myArray) {
    var result = "<table border=1 style=\"width: 784px\">";

    for (var i = 0; i < myArray.length; i++) {
        if (myArray[i][3] == "On") {
            let date = new Date(myArray[i][0]);

            result += '<tr style="background-color: green">';
            result += "<td>" + date + "</td>";
            result += "<td>" + myArray[i][1] + "</td>";
            result += "<td>" + myArray[i][2] + "</td>";
            result += "<td>" + myArray[i][3] + "</td>";
            result += "</tr>";
        } else if (myArray[i][3] == "Off") {
            let date = new Date(myArray[i][0]);

            result += '<tr style="background-color: #FE4365">';
            result += "<td>" + date + "</td>";
            result += "<td>" + myArray[i][1] + "</td>";
            result += "<td>" + myArray[i][2] + "</td>";
            result += "<td>" + myArray[i][3] + "</td>";
            result += "</tr>";
        } else {
            result += "<tr>";
            result += "<td>" + myArray[i][0] + "</td>";
            result += "<td>" + myArray[i][1] + "</td>";
            result += "<td>" + myArray[i][2] + "</td>";
            result += "<td>" + myArray[i][3] + "</td>";
            result += "</tr>";
        }
    }
    result += "</table>";

    return result;
}