import http from 'k6/http';
import { check, fail, sleep } from 'k6';
import { open } from 'k6/experimental/fs';
import csv from 'k6/experimental/csv';
import { scenario } from 'k6/execution';

export const options = {
  // A number specifying the number of VUs to run concurrently.
  vus: 10,
  // A string specifying the total duration of the test run.
  duration: '30s',
  iterations: 10,
};

const apiEndpoint = __ENV.ENDPOINT_URL || 'http://localhost:8000';

let file;
let csvData; 
let cachedData = {}; 

(async function () {
  file = await open('wine.csv');
  csvData = await csv.parse(file, { delimiter: ',', skipFirstLine: true});
})();

function postData() {
  let text = csvData[Math.floor(Math.random() * csvData.length)];
  text = String(text).replace(/[^\x00-\x7F]/g, "");

  let res = http.post(`${apiEndpoint}/texts`, JSON.stringify({data: text}), {
    headers: {'Content-Type': 'application/json'},
  });

  let success = check(res, {
    'status code is 201': (r) => r.status === 201,
    'id exists in response': (r) => {
      let json = r.json();
      return json.id !== undefined && json.id !== null;
    },
  });

  if (success) {
    let returnedId = res.json().id;
    cachedData[returnedId] = text;
    console.log(`Stored id: ${returnedId}`);
  } else {
    fail(`Unexpected response (${res.status}): ${res.body}`);
  }
}

function deleteText() {
  if (Object.keys(cachedData).length < 1) {
    console.log("No cached data available for deletion.");
    return;
  }

  let keys = Object.keys(cachedData);
  let uuid = keys[Math.floor(Math.random() * keys.length)];
  let res = http.del(`${apiEndpoint}/texts/${uuid}`);

  let success = check(res, {
    'status was 204': (r) => r.status === 204,
  });

  if (!success) {
    fail(`Failed to delete element (${res.status}): ${res.body}`);
  } else {
    delete cachedData[uuid];
    console.log(`Deleted id: ${uuid}`);
  }
}

function getText() {
  if (Object.keys(cachedData).length < 1) {
    console.log("No cached data available for retrieval.");
    return;
  }

  let keys = Object.keys(cachedData);
  let uuid = keys[Math.floor(Math.random() * keys.length)];
  let res = http.get(`${apiEndpoint}/texts/${uuid}`);

  let success = check(res, {
    'status was 200': (r) => r.status === 200,
    'data matches cached data': (r) => {
      let json = r.json();
      return json.data === cachedData[uuid];
    },
  });

  if (!success) {
    fail(`Data returned not expected (${res.status}): ${res.body}`);
  } else {
    console.log(`Retrieved and verified data for id: ${uuid}`);
  }

}

function randomString(length=8) {
  let characters = "abcdefghijklmnopqrstuvwxyz0123456789";
  let result = "";
  for (let i = 0; i < length; i++) {
      result += characters.charAt(Math.floor(Math.random() * characters.length));
  }
  return result;
}

function searchText() {
  let keys = Object.keys(cachedData);
  if (keys < 1) {
    console.log("No cached data available to search.");
    return;
  }

  let uuid = keys[Math.floor(Math.random() * keys.length)];
  let text = cachedData[uuid];
  let word = "";

  while (word === "") {
    let words = text.split(" ");
    word = words[Math.floor(Math.random() * words.length)];
  }

  let exists = true;
  if (Math.random() > 0.5) {
    exists = false;
    word = randomString();
  }

  let encodedWord = encodeURIComponent(word);
  let res = http.get(`${apiEndpoint}/texts/${uuid}/search?term=${encodedWord}`);

  let success = check(res, {
    'status was 200': (r) => r.status === 200,
    'correct search result': (r) => {
      let json = r.json();
      return json.found === exists;
    },
  });

  if (!success) {
    fail(`Incorrect response for search term '${word}' in '${text}': ${res.body}`);
  } else {
    console.log(`Searched for term '${word}' in text with id: ${uuid}, expected found: ${exists}`);
  }
}

// The function that defines VU logic.
//
// See https://grafana.com/docs/k6/latest/examples/get-started-with-k6/ to learn more
// about authoring k6 scripts.
//
export default function () {
  postData();
  getText();
  searchText();
  deleteText();

}
