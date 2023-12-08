const axios = require("axios");

async function evaluateResultSingleInput(input, relevantResult){
    response = await axios.post("http://localhost:8080/match", {
        query: input,
    })
    console.log(response.data);
    for(let i = 0; i < response.data.length; i++){
        if (response.data[i].movie.titleid === relevantResult){
            return (1/(i+1));
        }
    }
    return 0;
}

async function evaluateSingleResults(){
    testData = [
        {query: "batman begins", result: "tt0372784"},
        {query: "eternal sunshine of the spotless mind", result: "tt0338013"},
        {query: "attack of the killer manatee", result: "tt1979186"},
        {query: "aquamarine", result: "tt0429591"},
        {query: "12 angry men henry fonda", result: "tt0050083"},
        {query: "godzilla michael dougherty", result: "tt3741700"},
        {query: "brad pitt leonardo di caprio",result: "tt7131622"},
        {query: "parasite bong joon ho", result: "tt6751668"},
        {query: "2001 space odyssey", result: "tt0062622"},
        {query: "the godfather", result: "tt0068646"}
    ]
    recall = 0;
    result = 0;
    for (let record of testData){
        newresult = await evaluateResultSingleInput(record.query, record.result);
        result += newresult;
        if (newresult > 0){
            recall += 1
        }
    }
    result = {"MRR":result/testData.length, "Recall": recall/testData.length};
    return result;
}


async function main(){
    result = await evaluateSingleResults();
    console.log(result);
}

main();



