/*****************************************************************//**
 * \file   core.hpp
 * \brief  Header to include all necessary headers for basic usage and
 * also necessary functions to import opack into a flecs.

 * \author Tristan de Blauwe
 * \date   August 2022
 * \copyright 2024 CNRS - French National center for scientific research
 * \copyright 2024 UTC - University of technology of Compiègne
 * \copyright 2024 UPS - University of Paris Saclay
 *********************************************************************/
#pragma once

#include <opack/core/api_types.hpp>
#include <opack/core/components.hpp>
#include <opack/core/world.hpp>
#include <opack/core/entity.hpp>
#include <opack/core/action.hpp>
#include <opack/core/perception.hpp>
#include <opack/core/communication.hpp>
#include <opack/core/simulation.hpp>
#include <opack/core/operation.hpp>
#include <opack/utils/debug.hpp>

namespace opack
{
    /** @brief Create a world with opack imported. */
    World create_world();

    /** @brief Import opack into an existing world. */
    void import_opack(World& world);
}



