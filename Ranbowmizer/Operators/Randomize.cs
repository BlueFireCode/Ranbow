using System;
using System.Linq;
using System.Collections.Generic;

using RanbowBack.Enums;
using RanbowBack.Models;

namespace Ranbowmizer.Operators
{
	public class Randomize
	{
		public static OperatorModel RandomizeOperator(List<OperatorModel> operators)
		{
			Random random = new();
			int num = random.Next(0, operators.Count);
			OperatorModel op = operators[num];
			OperatorModel ret = new()
			{
				ID = op.ID,
				Name = op.Name,
				Description = op.Description
			};

			List<WeaponModel> primaries = new();
			primaries.Add(op.Primary1);
			if (op.Primary2 is not null)
				primaries.Add(op.Primary2);
			if (op.Primary3 is not null)
				primaries.Add(op.Primary3);
			ret.Primary1 = RandomizeWeapon(primaries);

			List<WeaponModel> secondaries = new();
			secondaries.Add(op.Secondary1);
			if (op.Secondary2 is not null)
				secondaries.Add(op.Secondary2);
			if (op.Secondary3 is not null)
				secondaries.Add(op.Secondary3);
			ret.Secondary1 = RandomizeWeapon(secondaries);

			ret.Gadget1 = random.Next(0, 2) == 0 ? op.Gadget1 : op.Gadget2;

			return ret;
		}

		public static WeaponModel RandomizeWeapon(List<WeaponModel> weapons)
		{
			Random random = new();
			WeaponModel wep;
			if (weapons.Count > 1)
				wep = weapons[random.Next(0, weapons.Count)];
			else
				wep = weapons.FirstOrDefault();
			WeaponModel ret = new()
			{
				ID = wep.ID,
				Name = wep.Name
			};

			ret.Sights = wep.Sights.Count > 0 ? new List<Sights> { wep.Sights[random.Next(0, wep.Sights.Count)] }: null;
			ret.Barrels = wep.Barrels.Count > 0 ? new List<Barrels> { wep.Barrels[random.Next(0, wep.Barrels.Count)] } : null;
			ret.Grips = wep.Grips.Count > 0 ? new List<Grips> { wep.Grips[random.Next(0, wep.Grips.Count)] } : null;
			ret.Laser = wep.Laser && (random.Next(0, 2) != 0);

			return ret;
		}
	}
}
