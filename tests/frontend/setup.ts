import { beforeAll, afterEach, afterAll } from 'vitest';
import { cleanup } from '@testing-library/svelte';

beforeAll(() => {
  console.log('Starting frontend test suite');
});

afterEach(() => {
  cleanup();
});

afterAll(() => {
  console.log('Frontend test suite completed');
});
