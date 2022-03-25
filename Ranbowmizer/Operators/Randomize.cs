using RanbowBack.Enums;
using RanbowBack.Models;
using System;
using System.Collections.Generic;
using System.Linq;

namespace Ranbowmizer.Operators
{
    public class Randomize
    {
        public static OperatorModel RandomizeOperator(List<OperatorModel> operators, bool skipGonne6)
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
            ret.Primary1 = RandomizeWeapon(primaries, skipGonne6);

            List<WeaponModel> secondaries = new();
            secondaries.Add(op.Secondary1);
            if (op.Secondary2 is not null)
                secondaries.Add(op.Secondary2);
            if (op.Secondary3 is not null)
                secondaries.Add(op.Secondary3);
            ret.Secondary1 = RandomizeWeapon(secondaries, skipGonne6);

            ret.Gadget1 = random.Next(0, 2) == 0 ? op.Gadget1 : op.Gadget2;

            return ret;
        }

        public static WeaponModel RandomizeWeapon(List<WeaponModel> weapons, bool skipGonne6)
        {
            Random random = new();
            WeaponModel wep;
            if (skipGonne6)
            {
                if (weapons.Count > 1 && !weapons.Any(x => x.Name == "GONNE-6"))
                    wep = weapons[random.Next(0, weapons.Count)];
                else
                    wep = weapons.FirstOrDefault(x => x.Name != "GONNE-6");
            }
            else
            {
                if (weapons.Count > 1)
                    wep = weapons[random.Next(0, weapons.Count)];
                else
                    wep = weapons.FirstOrDefault();
            }
            WeaponModel ret = new()
            {
                ID = wep.ID,
                Name = wep.Name
            };

            var sightOptions = Enum.GetValues(typeof(Sights)).Cast<Sights>().Where(x => wep.Sights.HasFlag(x)).ToArray();
            ret.Sights = (byte)wep.Sights > 1 ? sightOptions[random.Next(0, sightOptions.Length)] : Sights.Iron_Sights;
            var barrelOptions = Enum.GetValues(typeof(Barrels)).Cast<Barrels>().Where(x => wep.Barrels.HasFlag(x)).ToArray();
            ret.Barrels = (byte)wep.Barrels > 1 ? barrelOptions[random.Next(0, barrelOptions.Length)] : Barrels.Blank;
            var gripOptions = Enum.GetValues(typeof(Grips)).Cast<Grips>().Where(x => wep.Grips.HasFlag(x)).ToArray();
            ret.Grips = (byte)wep.Grips > 1 ? gripOptions[random.Next(0, gripOptions.Length)] : Grips.Blank;
            ret.Laser = wep.Laser && (random.Next(0, 2) != 0);

            return ret;
        }
    }
}
