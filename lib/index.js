'use strict';

const path = require('path');
const ackermannNeon = require(path.resolve(__dirname, '..', 'native'));

module.exports = {
  ack: (m, n) => ackermannNeon.ack(m, n),
};
