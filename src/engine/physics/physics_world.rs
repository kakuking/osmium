use rapier3d::prelude::*;

pub struct PhysicsWorld {
    pub gravity: Vector<Real>,

    pub pipeline: PhysicsPipeline,
    pub integration_parameters: IntegrationParameters,

    pub islands: IslandManager,
    pub broad_phase: DefaultBroadPhase,
    pub narrow_phase: NarrowPhase,

    pub bodies: RigidBodySet,
    pub colliders: ColliderSet,

    pub impulse_joints: ImpulseJointSet,
    pub multibody_joints: MultibodyJointSet,

    pub ccd_solver: CCDSolver,
}

impl PhysicsWorld {
    pub fn new() -> Self {
        Self {
            gravity: vector![0.0, -9.81, 0.0],

            pipeline: PhysicsPipeline::new(),
            integration_parameters: IntegrationParameters::default(),

            islands: IslandManager::new(),
            broad_phase: DefaultBroadPhase::new(),
            narrow_phase: NarrowPhase::new(),

            bodies: RigidBodySet::new(),
            colliders: ColliderSet::new(),

            impulse_joints: ImpulseJointSet::new(),
            multibody_joints: MultibodyJointSet::new(),

            ccd_solver: CCDSolver::new(),
        }
    }

    pub fn step(&mut self, dt: f32) {
        self.integration_parameters.dt = dt;

        self.pipeline.step(
            &self.gravity,
            &self.integration_parameters,
            &mut self.islands,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.bodies,
            &mut self.colliders,
            &mut self.impulse_joints,
            &mut self.multibody_joints,
            &mut self.ccd_solver,
            &(),
            &(),
        );
    }
}