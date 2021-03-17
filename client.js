const http2 = require('http2');
const client = http2.connect('http://localhost:3001');
const req = client.request({ 
':scheme': 'http',
  ':path': '/helloworld.Greeter/SayHello',
  ':method': 'POST',
//   ':authority': 'localhost:8443',
  passport: 'lishenggen_passport',
  'grpc-accept-encoding': 'identity,deflate,gzip',
  'accept-encoding': 'identity,gzip',
  'user-agent': 'grpc-node-js/1.2.9',
  'content-type': 'application/grpc',
});
req.on('response', (headers, flags) => {
  console.log(headers);
});

req.on('trailers', (headers, flags) => {
    console.log('trailers',headers, flags)
})

req.on('data', (data, flags) => {
    console.log(data, flags);
  });
req.end(bufStrToBuf('00 00 00 00 07 0a 05 77 6f 72 6c 64'))

function bufStrToBuf(str) {
    let temp = str.split(' ').map(i => parseInt(i, 16));
    return Buffer.from(temp);
}