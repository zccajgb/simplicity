let { ObjectId } = require('mongodb');

module.exports = {
  async up(db, client) {
    let tasks = await db.collection('tasks').find({ _id: new ObjectId("6786a2e31e37256eff04076a") }).toArray();
    let bulkOps = tasks.map(task => {
      let task_order = task.last_updated.getTime();
      return { updateOne: { filter: { _id: task._id }, update: { $set: { order: task_order } } } };
    });

    await db.collection('tasks').bulkWrite(bulkOps);
  },

  async down(db, client) {
    await db.collection('tasks').updateMany({}, { $unset: { order: '' } });
  }
};
