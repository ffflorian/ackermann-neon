'use strict';

const ackermannNeon = require('../lib');

describe('AckermannNeon', () => {
  it('calculates ack(3, 1)', () => {
    expect(ackermannNeon.ack(3, 1)).toBe(13);
  });
});
