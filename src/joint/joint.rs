#![allow(missing_docs)] // For downcast.

use downcast_rs::Downcast;
use na::{DVectorSliceMut, RealField};

use crate::object::{Multibody, MultibodyLink, BodyPartHandle};
use crate::math::{Isometry, JacobianSliceMut, Vector, Velocity};
use crate::solver::{ConstraintSet, GenericNonlinearConstraint, IntegrationParameters};

/// Trait implemented by all joints following the reduced-coordinate formation.
pub trait Joint<N: RealField>: Downcast + Send + Sync {
    /// The number of degrees of freedom allowed by the joint.
    fn ndofs(&self) -> usize;
    /// The position of the multibody link containing this joint relative to its parent.
    fn body_to_parent(&self, parent_shift: &Vector<N>, body_shift: &Vector<N>) -> Isometry<N>;
    /// Update the jacobians of this joint.
    fn update_jacobians(&mut self, body_shift: &Vector<N>, vels: &[N]);
    /// Integrate the position of this joint.
    fn integrate(&mut self, parameters: &IntegrationParameters<N>, vels: &[N]);
    /// Apply a displacement to the joint.
    fn apply_displacement(&mut self, disp: &[N]);

    /// Sets in `out` the non-zero entries of the joint jacobian transformed by `transform`.
    fn jacobian(&self, transform: &Isometry<N>, out: &mut JacobianSliceMut<N>);
    /// Sets in `out` the non-zero entries of the time-derivative of the joint jacobian transformed by `transform`.
    fn jacobian_dot(&self, transform: &Isometry<N>, out: &mut JacobianSliceMut<N>);
    /// Sets in `out` the non-zero entries of the velocity-derivative of the time-derivative of the joint jacobian transformed by `transform`.
    fn jacobian_dot_veldiff_mul_coordinates(
        &self,
        transform: &Isometry<N>,
        vels: &[N],
        out: &mut JacobianSliceMut<N>,
    );

    /// Multiply the joint jacobian by generalized velocities to obtain the
    /// relative velocity of the multibody link containing this joint.
    fn jacobian_mul_coordinates(&self, vels: &[N]) -> Velocity<N>;
    /// Multiply the joint jacobian by generalized accelerations to obtain the
    /// relative acceleration of the multibody link containing this joint.
    fn jacobian_dot_mul_coordinates(&self, vels: &[N]) -> Velocity<N>;

    /// Fill `out` with the non-zero entries of a damping that can be applied by default to ensure a good stability of the joint.
    fn default_damping(&self, out: &mut DVectorSliceMut<N>);

    /// The maximum number of impulses needed by this joints for
    /// its constraints.
    fn nimpulses(&self) -> usize {
        // FIXME: keep this?
        self.ndofs() * 3
    }

    /// Maximum number of velocity constrains that can be generated by this joint.
    fn num_velocity_constraints(&self) -> usize {
        0
    }

    /// Initialize and generate velocity constraints to enforce, e.g., joint limits and motors.
    fn velocity_constraints(
        &self,
        _params: &IntegrationParameters<N>,
        _multibody: &Multibody<N>,
        _link: &MultibodyLink<N>,
        _assembly_id: usize,
        _dof_id: usize,
        _ext_vels: &[N],
        _ground_j_id: &mut usize,
        _jacobians: &mut [N],
        _velocity_constraints: &mut ConstraintSet<N, (), (), usize>,
    ) {}

    /// The maximum number of non-linear position constraints that can be generated by this joint.
    fn num_position_constraints(&self) -> usize {
        0
    }

    /// Initialize and generate the i-th position constraints to enforce, e.g., joint limits.
    fn position_constraint(
        &self,
        _i: usize,
        _multibody: &Multibody<N>,
        _link: &MultibodyLink<N>,
        _handle: BodyPartHandle<()>,
        _dof_id: usize,
        _jacobians: &mut [N],
    ) -> Option<GenericNonlinearConstraint<N, ()>> {
        None
    }

    fn clone(&self) -> Box<Joint<N>>;
}

impl_downcast!(Joint<N> where N: RealField);
