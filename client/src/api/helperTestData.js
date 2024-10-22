export function fakeProjects() {
  return [
    { name: 'inbox', id: 0 },
    { name: 'olja', id: 1 },
    { name: 'home', id: 2 },
    { name: 'work', id: 3 },
  ];
}

export function fakeTags() {
  return [
    { name: 'important', id: 1 },
    { name: 'urgent', id: 2 },
    { name: 'fun', id: 3 },
    { name: 'boring', id: 4 },
  ];
}

export function fakeTasks() {
  return [
    { 
      ttl: 'today',
      name: 'Cuddle Olja',
      done: false,
      project: 1,
      tags: [1, 2]
    },
    { 
      ttl: 'tomorrow',
      name: 'Make Tea',
      done: false,
      project: 2,
      tags: [1, 3]
    },
    { 
      ttl: 'later',
      name: 'Go to Sleep',
      done: true,
      project: 0,
      tags: [2, 3]
    },
    {
      ttl: 'today',
      name: 'Buy Milk',
      done: false,
      project: 2,
      tags: [1, 2]
    },
    { 
      ttl: 'tomorrow',
      name: 'Go to the Gym',
      done: false,
      project: 3,
      tags: [1, 4]
    },
    { 
      ttl: 'later',
      name: 'Eat Breakfast',
      done: true,
      project: 3,
      tags: [4]
    },
    {
      ttl: 'today',
      name: 'Buy Eggs',
      done: false,
      project: 0,
      tags: [1]
    },
    { 
      ttl: 'tomorrow',
      name: 'Go to the Pool',
      done: false,
      project: 2,
      tags: [2]
    },
    { 
      ttl: 'later',
      name: 'Eat Lunch',
      done: true,
      project: 3,
      tags: [3]
    },
    {
      ttl: 'today',
      name: 'Buy Bread',
      done: false,
      project: 1,
      tags: []
    },
    { 
      ttl: 'tomorrow',
      name: 'Go to the Park',
      done: false,
      project: 3,
      tags: [4]
    },
    { 
      ttl: 'later',
      name: 'Eat Dinner',
      done: true,
      project: 1,
      tags: []
    },
    {
      ttl: 'today',
      name: 'Buy Butter',
      done: false,
      project: 0,
      tags: []
    },
    { 
      ttl: 'tomorrow',
      name: 'Go to the Beach',
      done: false,
      project: 0,
      tags: []
    },
    { 
      ttl: 'later',
      name: 'Eat Snacks',
      done: true,
      project: 0,
      tags: []
    },
    {
      ttl: 'today',
      name: 'Buy Cheese',
      done: false,
      project: 0,
      tags: []
    },
    { 
      ttl: 'tomorrow',
      name: 'Go to the Movies',
      done: false,
      project: 1,
      tags: []
    },
    { 
      ttl: 'later',
      name: 'Eat Dessert',
      done: true,
      project: 3,
      tags: []
    },
    {
      ttl: 'today',
      name: 'Buy Jam',
      done: false,
      project: 3,
      tags: []
    },
    { 
      ttl: 'tomorrow',
      name: 'Go to the Mall',
      done: false,
      project: 3,
      tags: []
    },
    { 
      ttl: 'later',
      name: 'Eat Supper',
      done: true,
      project: 0,
      tags: []
    },
    {
      ttl: 'today',
      name: 'Buy Honey',
      done: false,
      project: 0,
      tags: []
    },
    { 
      ttl: 'tomorrow',
      name: 'Go to the Zoo',
      done: false,
      project: 2,
      tags: []
    },
    { 
      ttl: 'later',
      name: 'Eat Midnight Snack',
      done: true,
      project: 2,
      tags: []
    },
    {
      ttl: 'today',
      name: 'Buy Sugar',
      done: false,
      project: 0,
      tags: []
    },
    { 
      ttl: 'tomorrow',
      name: 'Go to the Circus',
      done: false,
      project: 1,
      tags: []
    },
    { 
      ttl: 'later',
      name: 'Eat Brunch',
      done: false,
      project: 3,
      tags: []
    },
  ]
}
