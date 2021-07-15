using MySql.Data.MySqlClient;
using RanbowBack.DbObjects;
using RanbowBack.Models;
using RanbowBack.Repositories.Base;
using System;
using System.Diagnostics;

namespace RanbowBack.Repositories
{
	public class GadgetRepository : BaseRepository
	{
		public GadgetRepository(string connectionString) : base(connectionString) { }

		public bool Read(int id, out GadgetModel entity)
		{
			bool result = false;
			DbGadget dbitem = null;

			try
			{
				DbCommand.CommandText = "SELECT * FROM Gadget WHERE ID = @id;";
				DbCommand.Parameters.Clear();
				DbCommand.Parameters.AddWithValue("id", id);
				DbConnection.Open();
				using (MySqlDataReader reader = DbCommand.ExecuteReader())
				{
					reader.Read();
					dbitem = new()
					{
						ID = id,
						Name = (string)reader["Name"]
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
				entity = new()
				{
					ID = dbitem.ID,
					Name = dbitem.Name
				};
				return result;
			}
			entity = null;
			return false;
		}
	}
}
