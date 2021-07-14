using MySql.Data.MySqlClient;
using RanbowBack.DbObjects;
using RanbowBack.Enums;
using RanbowBack.Models;
using RanbowBack.Repositories.Base;
using System;
using System.Collections.Generic;
using System.Diagnostics;

namespace RanbowBack.Repositories
{
	class WeaponRepository : BaseRepository
	{
		public WeaponRepository(string connectionString) : base(connectionString){ }

		public bool Read(int id, out WeaponModel entity)
		{
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
						ID = id,
						Name = (string)reader["Name"],
						Sights = (string)reader["Sight"],
						Barrels = (string)reader["Barrel"],
						Grips = (string)reader["Grip"],
						Laser = (bool)reader["Laser"]
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
				var sightArr = dbitem.Sights.Split(';');
				foreach (var item in sightArr)
					sights.Add((Sights)int.Parse(item));

				List<Barrels> barrels = new();
				var barrelArr = dbitem.Barrels.Split(';');
				foreach (var item in barrelArr)
					barrels.Add((Barrels)int.Parse(item));

				List<Grips> grips = new();
				var gripArr = dbitem.Grips.Split(';');
				foreach (var item in gripArr)
					grips.Add((Grips)int.Parse(item));

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
