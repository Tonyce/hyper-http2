const http2 = require('http2');
const server = http2.createServer();
server.on('stream', (stream, headers) => {
    console.log(headers)
  stream.respond({ ':status': 200 }, { waitForTrailers: true });
  stream.on('wantTrailers', () => {
    stream.sendTrailers({ ABC: 'some value to send' });
  });
  stream.end('some data');
});
server.listen(4567)