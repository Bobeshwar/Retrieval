const axios = require("axios");

async function evaluateResultSingleInput(input, relevantResult){
    const response = await axios.post("http://localhost:8080/match", {
        query: input,
    })
    for(let i = 0; i < response.data.length; i++){
        
        if (response.data[i].movie.titleid === relevantResult){
            return (1/(i+1));
        }
    }
    return 0;
}

async function evaluateResultMultipleOutput(input, relevantResults){
    const response = await axios.post("http://localhost:8080/match", {
        query: input,
    })
    const relevantSet = new Set(relevantResults);
    let relevant = 0;
    let nonrelevant = 0;
    let precision = 0;
    for (let i = 0; i < response.data.length; i++){
        // console.log(JSON.stringify(response.data[i]))
        if (relevantSet.has(response.data[i].movie.titleid)){
            relevant += 1;
        } else {
            nonrelevant += 1;
        }
        precision += (relevant/(relevant+nonrelevant));
        if (relevant >= relevantResults.length){
            break;
        }
    }
    return {'MAP': precision/(relevant+ nonrelevant), 'Recall': relevant/relevantResults.length}
}

async function evaluateResultMultipleOutputSimilarity(originalquery, title_id, genres, relevantResults){
    const response = await axios.post("http://localhost:8080/similar", {
        original_query: originalquery,
        genres: genres,
        title_id: title_id
    })
    const relevantSet = new Set(relevantResults);
    let relevant = 0;
    let nonrelevant = 0;
    let precision = 0;
    for (let i = 0; i < response.data.length; i++){
        // console.log(JSON.stringify(response.data[i]));
        if (relevantSet.has(response.data[i].movie.titleid)){
            relevant += 1;
        } else {
            nonrelevant += 1;
        }
        precision += (relevant/(relevant+nonrelevant));
        if (relevant >= relevantResults.length){
            break;
        }
    }
    return {'MAP': precision/(relevant+ nonrelevant), 'Recall': relevant/relevantResults.length}
}

async function evaluateMultipleResultsSimilarity(){
    testData = [
        {original_query: "deathly hallows", genres : ["Adventure","Family","Fantasy"], title_id : "tt0926084", results:["tt0241527","tt0295297","tt0330373","tt0304141", "tt0373889","tt0417741","tt0926084","tt1201607"]},
        {original_query: "avengers", genres: ["Action","Sci-Fi"], title_id: "tt0848228", results:["tt0848228", "tt4154756", "tt2395427", "tt4154796", "tt0458339", "tt1843866","tt0371746", "tt1228705", "tt0800080", "tt0800369"]},
        {original_query: "inception", genres: ["Action","Adventure","Sci-Fi"], title_id: "tt1375666", results:["tt0816692","tt0468569","tt0109830", "tt0137523"]},

    ]
    recall = 0;
    map = 0;
    for (let record of testData){
        const newresult = await evaluateResultMultipleOutputSimilarity(record.original_query,  record.title_id, record.genres, record.results);
        recall += newresult.Recall;
        map += newresult.MAP;
    }
    const finalResult = {"MAP": map/testData.length, "Recall": recall/testData.length};
    return finalResult;
}

async function evaluateMultipleResults(){
    testData = [
        {query: "chronicles of narnia", results: ["tt0363771","tt0499448","tt0980970"]},
        {query: "harry potter", results: ["tt0241527","tt0295297","tt0330373","tt0304141", "tt0373889","tt0417741","tt0926084","tt1201607"]},
        {query: "karan johar", results: ["tt14993250","tt10230404", "tt8439854", "tt4559006", "tt2797242", "tt2172071", "tt1188996", "tt0449999", "tt0248126", "tt0172684"]},
        {query: "wind waker", results: ["tt0325724", "tt3206522"]},
        {query: "daniel radcliffe emma watson rupert grint", results: ["tt0241527","tt0295297","tt0330373","tt0304141", "tt0373889","tt0417741","tt0926084","tt1201607"]},
        {query: "joe pesci scorsese", results: ["tt1302006","tt0081398","tt0099685", "tt0112641", "tt11353562"]},
        {query: "christopher nolan", results: ["tt15398776","tt0468569", "tt0816692","tt1375666","tt6723592","tt5013056","tt0209144","tt1345836","tt0372784","tt0482571"]}
    ]
    recall = 0;
    map = 0;
    for (let record of testData){
        const newresult = await evaluateResultMultipleOutput(record.query, record.results);
        console.log(record.query, JSON.stringify(newresult));
        recall += newresult.Recall;
        map += newresult.MAP;
    }
    const finalResult = {"MAP": map/testData.length, "Recall": recall/testData.length};
    return finalResult;
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
        {query: "the godfather", result: "tt0068646"},
        {query: "Pardessus économique", result: "tt3978256"}
    ]
    let recall = 0;
    let result = 0;
    for (let record of testData){
        const newresult = await evaluateResultSingleInput(record.query, record.result);
        result += newresult;
        if (newresult > 0){
            recall += 1
        }
    }
    const finalResult = {"MRR":result/testData.length, "Recall": recall/testData.length};
    return finalResult;
}


async function main(){
    let result = await evaluateSingleResults();
    console.log(result);
    result = await evaluateMultipleResults();
    console.log(result);
    result = await evaluateMultipleResultsSimilarity();
    console.log(result);
}

main();



