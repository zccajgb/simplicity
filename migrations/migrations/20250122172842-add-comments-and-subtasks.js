module.exports = {
  async up(db, client) {
    await db.collection("tasks").updateMany({ comments: { "$exists": false } }, { $set: { comments: "" } });
    await db.collection("tasks").updateMany({ subtasks: { "$exists": false } }, { $set: { subtasks: [] } });
  },

  async down(db, client) {
    // TODO write the statements to rollback your migration (if possible)
    // Example:
    // await db.collection('albums').updateOne({artist: 'The Beatles'}, {$set: {blacklisted: false}});
  }
};
