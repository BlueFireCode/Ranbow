using System;
using System.Diagnostics;
using System.Collections.Generic;

using MySql.Data.MySqlClient;

using RanbowBack.Enums;
using RanbowBack.Models;
using RanbowBack.DbObjects;
using RanbowBack.Repositories.Base;

namespace RanbowBack.Repositories
{
	public class WeaponRepository : BaseRepository
	{
		public WeaponRepository(string connectionString) : base(connectionString){ }

		public bool Read(int? id, out WeaponModel entity)
		{
			if (id is null)
			{
				entity = null;
				return true;
			}
			bool result = false;
			DbWeapon dbitem = null;

			try
			{
				DbCommand.CommandText = "SELECT * FROM Weapon WHERE ID = @id;";
				DbCommand.Parameters.Clear();
				DbCommand.Parameters.AddWithValue("id", id);
				DbConnection.Open();
				using (MySqlDataReader reader = DbCommand.ExecuteReader())
				{
					reader.Read();
					dbitem = new()
					{
						ID = (int)id,
						Name = (string)reader["Name"],
#nullable enable
						Sights = Convert.IsDBNull(reader["Sight"]) ?  null : (string?)reader["Sight"],
						Barrels = Convert.IsDBNull(reader["Barrel"]) ? null : (string?)reader["Barrel"],
						Grips = Convert.IsDBNull(reader["Grip"]) ? null : (string?)reader["Grip"],
#nullable disable
						Laser = (ulong)reader["Laser"] != 0
					};
				}
				DbConnection.Close();

				result = true;
			}
			catch (Exception e)
			{
				//TODO: Add logging
				Debug.WriteLine(e.Message);
			}
			finally
			{
				if (DbConnection.State == System.Data.ConnectionState.Open)
				{
					DbConnection.Close();
				}
			}
			if (result && dbitem is not null)
			{
				List<Sights> sights = new();
				if (dbitem.Sights is not null)
				{
					var sightArr = dbitem.Sights.Split(';');
					foreach (var item in sightArr)
						sights.Add((Sights)int.Parse(item));
					sights.Add(Sights.Iron_Sights);
				}


				List<Barrels> barrels = new();
				if (dbitem.Barrels is not null)
				{
					var barrelArr = dbitem.Barrels.Split(';');
					foreach (var item in barrelArr)
						barrels.Add((Barrels)int.Parse(item));
					barrels.Add(Barrels.Blank);
				}

				List<Grips> grips = new();
				if (dbitem.Grips is not null)
				{
					var gripArr = dbitem.Grips.Split(';');
					foreach (var item in gripArr)
						grips.Add((Grips)int.Parse(item));
					grips.Add(Grips.Blank);
				}

				entity = new()
				{
					ID = dbitem.ID,
					Name = dbitem.Name,
					Sights = sights,
					Barrels = barrels,
					Grips = grips,
					Laser = dbitem.Laser
				};
				return result;
			}
			entity = null;
			return false;
		}
	}
}
