use crate::common::WrappedRcRefCell;
use crate::server::job::{Job, JobStatus};
use crate::{TaskId, WorkerId, Map};
use tako::messages::gateway::{TaskFailedMessage, TaskUpdate, NewWorkerMessage, LostWorkerMessage};
use crate::server::worker::Worker;
use tako::messages::worker::NewWorkerMsg;

pub struct State {
    jobs: crate::Map<TaskId, Job>,
    workers: crate::Map<WorkerId, Worker>,
    id_counter: TaskId,
}

pub type StateRef = WrappedRcRefCell<State>;

/*pub fn new_state_ref() -> StateRef {
        WrappedRcRefCell::wrap(State {

        })
}*/

impl State {
    pub fn jobs(&self) -> impl Iterator<Item=&Job> {
        self.jobs.values()
    }

    pub fn add_worker(&mut self, worker: Worker) {
        let worker_id = worker.worker_id();
        assert!(self.workers.insert(worker_id, worker).is_none())
    }

    pub fn add_job(&mut self, job: Job) {
        let task_id = job.task_id;
        assert!(self.jobs.insert(task_id, job).is_none())
    }

    pub fn new_job_id(&mut self) -> TaskId {
        let id = self.id_counter;
        self.id_counter += 1;
        id
    }

    pub fn get_workers(&self) -> &Map<WorkerId, Worker> {
        &self.workers
    }

    pub fn get_worker_mut(&mut self, worker_id: WorkerId) -> Option<&mut Worker> {
        self.workers.get_mut(&worker_id)
    }

    pub fn process_task_failed(&mut self, msg: TaskFailedMessage) {
        log::debug!("Task id={} failed", msg.id);
        let job = self.jobs.get_mut(&msg.id).unwrap();
        job.status = JobStatus::Failed(msg.info.message);
    }

    pub fn process_task_update(&mut self, msg: TaskUpdate) {
        log::debug!("Task id={} updated", msg.id);
        let job = self.jobs.get_mut(&msg.id).unwrap();
        job.status = JobStatus::Finished;
    }

    pub fn process_worker_new(&mut self, msg: NewWorkerMessage) {
        log::debug!("New worker id={}", msg.worker_id);
        self.add_worker(Worker::new(msg.worker_id, msg.configuration));
    }

    pub fn process_worker_lost(&mut self, msg: LostWorkerMessage) {
        log::debug!("Worker lost id={}", msg.worker_id);
        let mut worker = self.workers.get_mut(&msg.worker_id).unwrap();
        worker.set_offline_state();
    }


}

impl StateRef {
    pub fn new() -> StateRef {
        WrappedRcRefCell::wrap(State {
            jobs: Default::default(),
            workers: Default::default(),
            id_counter: 1,
        })
    }
}