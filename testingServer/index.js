const axios = require("axios");

async function evaluateResult(input, relevantResults){
    response = await axios.post("http://localhost:8080/match", {
        words: ["batman", "begins"],
    })
    console.log(response);
}

evaluateResult({},{})