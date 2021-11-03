updatePage();

document.getElementById("minuteInput").value = 5;

(function loop() {
	setTimeout(async function () {
        var width = document.getElementById("mainDiv").offsetWidth;

        // For some reason I came back and the graph height was broken. I need to come back around and fix this...
        var height =  500;

        var response = await fetch("/graph_" +document.getElementById("minuteInput").value+"_"+width+"x"+height+".svg");
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
    updatePage();
};

document.getElementById("toggle").onclick = async function() {
    var response = await fetch("/toggle");
    var json = await response.json();

    if (json.error != null) {
        alert("Couldn't toggle the pin! Error: " + json.error);
    };

    updatePage();
};

document.getElementById("refresh").onclick = async function() {
    updatePage();
}

async function updatePage() {
    var response = await fetch("/data.csv");
    var csvText = await response.text();
    var csvHTML = makeTableHTML(CSVToArray(csvText));
    document.getElementById("csvData").innerHTML = csvHTML;

    var weatherResponse = await fetch("/weather");
    var weatherText = await weatherResponse.text();

    var stateResponse = await fetch("/state");
    var stateText = await stateResponse.json();

    document.getElementById("state").innerHTML = "State: " + stateText.state + ", " + weatherText;
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
        if (myArray[i][3] == "On") {
            let date = new Date(myArray[i][0]);

            result += '<tr style="background-color: #072ff7">';
            result += "<td>" + date + "</td>";
            result += "<td>" + myArray[i][1] + "</td>";
            result += "<td>" + myArray[i][2] + "</td>";
            result += "<td>" + myArray[i][3] + "</td>";
            result += "</tr>";
        } else if (myArray[i][3] == "Off") {
            let date = new Date(myArray[i][0]);

            result += '<tr style="background-color: #e02702">';
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