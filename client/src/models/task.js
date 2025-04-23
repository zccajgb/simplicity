export class Task {
  constructor(name, userId, completed, projectId, tags, date, snooze, repeat, order, subtasks, comments, last_updated, depends) {
    this.name = name;
    this.userId = userId;
    this.completed = completed;
    this.projectId = projectId;
    this.tags = tags;
    this.date = date;
    this.snooze = snooze;
    this.repeat = repeat;
    this.order = order;
    this.subtasks = subtasks || [];
    this.comments = comments || [];
    this.last_updated = last_updated || new Date();
    this.depends = depends || [];
  }
  static clone(task) {
    return new Task(
      task.name,
      task.userId,
      null,
      task.projectId,
      task.tags,
      task.date,
      task.snooze,
      task.repeat,
      task.order,
      task.subtasks,
      task.comments,
      new Date(),
      task.depends
    );
  }
}